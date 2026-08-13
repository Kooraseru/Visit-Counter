# Release Records

This directory contains authored Markdown used to prepare GitHub Releases. It
exists only on `source`; publication branches and repository payloads must not
include these files.

Release record filenames use `YYYY.MM.N-KIND.release.md`. The release ID portion
uses the same monthly sequence and one of `regular`, `hotfix`, or `security`:

```text
2026.08.1-regular.release.md
```

Each record must use this structure:

```markdown
# 2026.08.1-regular

## Summary

A concise explanation of the release.

## Notable Changes

- Added a user-relevant capability ([#123](...)).
- Fixed a user-relevant problem.

## Issues Addressed

- Fixes [#118](...).

## Compatibility

Compatibility, migration, or rollback information. Write "No known
compatibility impact." when none is known.

## Verification

- A concrete validation result.
```

The first heading must exactly match the filename without `.release.md`. Keep
every required section non-empty. References in Notable Changes are encouraged
when meaningful but are not mandatory. Issues Addressed uses explicit issue
relationships such as `Fixes` or `Resolves` with links.

During publication, this authored record is prepended to GitHub's generated
release notes. GitHub supplies categorized change history, contributors, and a
comparison link. Do not include private vulnerability details in a security
release record; coordinate those details through the repository security
process.

GitHub workflow-dispatch choices are static. Add each release ID to the
`version` options in `.github/workflows/publish.yml` when adding its record.
Repository validation requires the dropdown choices and `.release.md` records
to match exactly.
