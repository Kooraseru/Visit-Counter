use std::{
    collections::BTreeMap,
    env, fs,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use image::{
    AnimationDecoder, Delay, Frame, ImageReader, RgbaImage,
    codecs::gif::{GifDecoder, GifEncoder, Repeat},
    imageops::{FilterType, overlay, resize},
};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const API_VERSION: &str = "2022-11-28";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Day {
    views: u64,
    uniques: u64,
}

#[derive(Debug, Deserialize)]
struct TrafficResponse {
    views: Vec<TrafficDay>,
}

#[derive(Debug, Deserialize)]
struct TrafficDay {
    timestamp: String,
    count: u64,
    uniques: u64,
}

struct Settings {
    repository: String,
    token: String,
    history: PathBuf,
    output: PathBuf,
    minimum_digits: usize,
    digit_width: u32,
    digit_height: u32,
}

impl Settings {
    fn from_environment(arguments: Vec<String>) -> Result<Self> {
        if arguments.len() != 6 {
            bail!("expected six action inputs, received {}", arguments.len());
        }
        let workspace = required("GITHUB_WORKSPACE")?;
        let repository = required("GITHUB_REPOSITORY")?;
        if repository.split_once('/').is_none() {
            bail!("GITHUB_REPOSITORY must be OWNER/REPOSITORY");
        }
        Ok(Self {
            repository,
            token: arguments[0].clone(),
            history: workspace_path(&workspace, &arguments[1])?,
            output: workspace_path(&workspace, &arguments[2])?,
            minimum_digits: positive_number("minimum-digits", &arguments[3])?,
            digit_width: positive_number("digit-width", &arguments[4])?,
            digit_height: positive_number("digit-height", &arguments[5])?,
        })
    }
}

fn main() {
    if let Err(error) = dispatch() {
        eprintln!("github-visit-counter: {error:#}");
        std::process::exit(1);
    }
}

fn dispatch() -> Result<()> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.first().map(String::as_str) == Some("render") {
        return render_command(&arguments[1..]);
    }
    run_action(arguments)
}

fn render_command(arguments: &[String]) -> Result<()> {
    if arguments.len() != 6 {
        bail!("render expects NUMBER OUTPUT DIGITS MINIMUM_DIGITS DIGIT_WIDTH DIGIT_HEIGHT");
    }
    let total = arguments[0]
        .parse::<u64>()
        .context("NUMBER must be a non-negative integer")?;
    let output = PathBuf::from(&arguments[1]);
    let digits = PathBuf::from(&arguments[2]);
    let minimum_digits = positive_number("MINIMUM_DIGITS", &arguments[3])?;
    let digit_width = positive_number("DIGIT_WIDTH", &arguments[4])?;
    let digit_height = positive_number("DIGIT_HEIGHT", &arguments[5])?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    if output.extension().and_then(|value| value.to_str()) != Some("gif") {
        bail!("render output must use the .gif extension");
    }
    render_gif(
        total,
        minimum_digits,
        digit_width,
        digit_height,
        &digits,
        &output,
    )?;
    println!("rendered {total} to {}", output.display());
    Ok(())
}

fn run_action(arguments: Vec<String>) -> Result<()> {
    let settings = Settings::from_environment(arguments)?;
    let mut history = load_history(&settings.history)?;
    merge_history(
        &mut history,
        fetch_traffic(&settings.repository, &settings.token)?,
    )?;
    let total = history.values().try_fold(0_u64, |sum, day| {
        sum.checked_add(day.views)
            .context("view total overflowed u64")
    })?;
    fs::create_dir_all(&settings.output)
        .with_context(|| format!("create {}", settings.output.display()))?;
    fs::write(
        settings.output.join("history.json"),
        format!("{}\n", serde_json::to_string_pretty(&history)?),
    )?;
    render_gif(
        total,
        settings.minimum_digits,
        settings.digit_width,
        settings.digit_height,
        &digit_directory(),
        &settings.output.join("views.gif"),
    )?;
    if let Ok(output) = env::var("GITHUB_OUTPUT") {
        use std::io::Write;
        writeln!(
            fs::OpenOptions::new().append(true).open(output)?,
            "total={total}"
        )?;
    }
    println!("recorded {total} views across {} days", history.len());
    Ok(())
}

fn digit_directory() -> PathBuf {
    env::var_os("VISIT_COUNTER_DIGITS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/opt/github-visit-counter/digits"))
}

fn fetch_traffic(repository: &str, token: &str) -> Result<Vec<TrafficDay>> {
    let response = Client::builder()
        .user_agent("github-visit-counter")
        .build()?
        .get(format!(
            "https://api.github.com/repos/{repository}/traffic/views?per=day"
        ))
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", API_VERSION)
        .send()
        .context("request GitHub Traffic API")?
        .error_for_status()
        .context("GitHub Traffic API rejected the request")?
        .json::<TrafficResponse>()?;
    Ok(response.views)
}

fn load_history(path: &Path) -> Result<BTreeMap<String, Day>> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))
}

fn merge_history(history: &mut BTreeMap<String, Day>, buckets: Vec<TrafficDay>) -> Result<()> {
    for bucket in buckets {
        let date = bucket
            .timestamp
            .get(..10)
            .context("traffic timestamp is shorter than a date")?;
        if !valid_date(date) {
            bail!("traffic timestamp has an invalid date: {date}");
        }
        history.insert(
            date.to_owned(),
            Day {
                views: bucket.count,
                uniques: bucket.uniques,
            },
        );
    }
    Ok(())
}

fn render_gif(
    total: u64,
    minimum: usize,
    width: u32,
    height: u32,
    digits: &Path,
    output: &Path,
) -> Result<()> {
    let value = format!("{total:0minimum$}");
    let canvas_width = width
        .checked_mul(value.len().try_into()?)
        .context("GIF width overflow")?;
    let animations = value
        .chars()
        .map(|digit| load_digit_frames(&digit_path(digits, digit)?, width, height))
        .collect::<Result<Vec<_>>>()?;
    let frame_count = animations.iter().map(Vec::len).max().unwrap_or(1);
    let mut frames = Vec::with_capacity(frame_count);
    for frame_index in 0..frame_count {
        let mut canvas = RgbaImage::new(canvas_width, height);
        let mut delay = Delay::from_numer_denom_ms(100, 1);
        for (digit_index, animation) in animations.iter().enumerate() {
            let frame = &animation[frame_index % animation.len()];
            if animation.len() > 1 {
                delay = frame.delay();
            }
            overlay(
                &mut canvas,
                frame.buffer(),
                i64::from(digit_index as u32 * width),
                0,
            );
        }
        frames.push(Frame::from_parts(canvas, 0, 0, delay));
    }
    let file = File::create(output).with_context(|| format!("create {}", output.display()))?;
    let mut encoder = GifEncoder::new(BufWriter::new(file));
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames.into_iter())?;
    Ok(())
}

fn load_digit_frames(path: &Path, width: u32, height: u32) -> Result<Vec<Frame>> {
    let mut frames = if path.extension().and_then(|value| value.to_str()) == Some("gif") {
        let decoder = GifDecoder::new(BufReader::new(File::open(path)?))?;
        decoder.into_frames().collect_frames()?
    } else {
        vec![Frame::new(ImageReader::open(path)?.decode()?.to_rgba8())]
    };
    for frame in &mut frames {
        if frame.buffer().dimensions() != (width, height) {
            let delay = frame.delay();
            let resized = resize(frame.buffer(), width, height, FilterType::Nearest);
            *frame = Frame::from_parts(resized, 0, 0, delay);
        }
    }
    if frames.is_empty() {
        bail!("counter image has no frames: {}", path.display());
    }
    Ok(frames)
}

fn digit_path(directory: &Path, digit: char) -> Result<PathBuf> {
    for extension in ["png", "jpg", "jpeg", "gif", "webp", "svg"] {
        let candidate = directory.join(format!("{digit}.{extension}"));
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    bail!("missing counter image for digit {digit}")
}

fn valid_date(value: &str) -> bool {
    value.len() == 10
        && value.bytes().enumerate().all(|(index, byte)| {
            if index == 4 || index == 7 {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
}

fn workspace_path(workspace: &str, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        bail!("action paths must be relative and cannot contain '..': {relative}");
    }
    Ok(Path::new(workspace).join(path))
}

fn required(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("required environment variable {name} is missing"))
}

fn positive_number<T>(name: &str, raw: &str) -> Result<T>
where
    T: std::str::FromStr + PartialOrd + From<u8>,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let value = raw.parse::<T>()?;
    if value <= T::from(0) {
        bail!("{name} must be positive");
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_days_are_replaced_without_losing_old_days() {
        let mut history = BTreeMap::from([
            (
                "2026-08-01".into(),
                Day {
                    views: 2,
                    uniques: 1,
                },
            ),
            (
                "2026-08-02".into(),
                Day {
                    views: 3,
                    uniques: 2,
                },
            ),
        ]);
        merge_history(
            &mut history,
            vec![
                TrafficDay {
                    timestamp: "2026-08-02T00:00:00Z".into(),
                    count: 8,
                    uniques: 4,
                },
                TrafficDay {
                    timestamp: "2026-08-03T00:00:00Z".into(),
                    count: 5,
                    uniques: 3,
                },
            ],
        )
        .unwrap();
        assert_eq!(history["2026-08-01"].views, 2);
        assert_eq!(history["2026-08-02"].views, 8);
        assert_eq!(history["2026-08-03"].views, 5);
    }

    #[test]
    fn parent_paths_are_rejected() {
        assert!(workspace_path("/github/workspace", "../escape").is_err());
    }
}
