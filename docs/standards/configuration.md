# Configuration Standards

Use the format required by an external consumer: YAML for GitHub Actions and
MkDocs, JSON when a consumer requires JSON, and TOML for project-owned
contributor-edited metadata when no external format is imposed.

Configuration must have a real consumer. Validate required keys, reject unknown
or unsafe values where practical, and fail with actionable messages. Do not add
empty registries, copied metadata, or machine-specific absolute paths.

Secrets belong in protected GitHub environments or repository secrets. They
must never appear in committed configuration, logs, generated artifacts, or
pull-request fixtures.
