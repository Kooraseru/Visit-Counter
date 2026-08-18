# Publication Configuration

Generated branches are built from the explicit whitelist in `config.yml`.
Publication is disabled until an adopting project sets `enabled: true` and
declares non-empty `include` and `required` lists. `channel_include` adds an
explicit per-channel layer; all three channel keys are required even when a
channel adds no files.

Every path is relative to the repository root. Entries must not be absolute,
contain `..`, resolve through symlinks outside the repository, overlap `.git`,
or include private local knowledge. Required entries must exist in every
payload. The stable channel allowlist contains only GitHub metadata and workflow
control-plane files that GitHub reads from the default branch; preview channels
receive no authored `.github` files beyond generated publication provenance.

A valid GitHub special filename is not sufficient: it must occupy a supported
path and, for repository-wide behavior, exist on the default branch. Keep the
stable allowlist explicit rather than publishing `.github/` or `workflows/`
recursively.

The builder creates `.github/publication.json` with the selected channel,
release ID, immutable source commit, and generation time. Release IDs use
`YYYY.MM.N-KIND`; the channel remains a separate publication destination. The
whitelist includes `src/` as a copyable `.github` installation package and
setup guide. The builder writes only beneath `.generated/repo/<channel>/`.
