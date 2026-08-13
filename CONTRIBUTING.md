# Contributing

Template changes should preserve clean boundaries between authored source,
private workspace state, generated output, and automation-owned branches.

> [!IMPORTANT]
> Change canonical inputs on `source`. Do not patch generated output or expose
> private agent, workspace, editor-setting, credential, or machine-local state.

## Before You Start

Read the [repository architecture](https://github.com/Kooraseru/Template/blob/source/docs/architecture/repository.md)
and the [project documentation](https://kooraseru.github.io/Template/).
Use Discussions for open-ended ideas or support questions. Use the appropriate
issue form for a reproducible defect or concrete proposal. Report
vulnerabilities privately through the repository Security tab.

## Branches

All contributor development happens on `source`.

Open issues, proposals, fixes, documentation changes, source changes, workflow
changes, and pull requests against `source`.

Do not author changes on `canary`, `beta`, or `stable`. Those branches are
generated publication outputs built from `source`:

<table>
  <tr>
    <td><code>source</code></td>
    <td>Canonical authoring branch</td>
  </tr>
  <tr>
    <td><code>canary</code></td>
    <td>Frequently moving generated channel for the earliest consumption</td>
  </tr>
  <tr>
    <td><code>beta</code></td>
    <td>Deliberately published generated testing channel</td>
  </tr>
  <tr>
    <td><code>stable</code></td>
    <td>Trusted generated production channel</td>
  </tr>
</table>

If generated output is wrong, fix its source-owned input on `source`, then
rebuild or republish it. Direct edits may be overwritten.

## Contribution Flow

1. Fork or branch from current `source`.
2. Keep the change focused on one agreed problem.
3. Update tests and project documentation with the implementation.
4. Run the same validation commands owned by GitHub Actions.
5. Open a pull request targeting `source`.
6. Include the problem, implementation, compatibility impact, and exact test results.
7. Address review feedback and keep the branch current until merge.

## Local Expectations

- Follow the ownership boundaries in `docs/architecture/repository.md`.
- Add configuration only when a real tool consumes it.
- Keep shared VS Code files portable; never commit `.vscode/settings.json`.
- Keep generated output beneath `.generated/` and out of authored commits.
- Add dependencies and validation stages only for implemented capabilities.
- Keep translation TOML beside the content it describes and register locales in
  `content/locales.toml`; do not create one canonical source file per locale.
- Add Pages layouts beneath `content/pages/` and generated repository layouts
  beneath `content/repo/`, then use `{{ l10n:component.context.key }}` at each
  translated fill-in area.

## Validation

Repository validation is owned by `.github/workflows/validate.yml`. Local checks
run the same scripts used by GitHub Actions; do not create local-only checks.
See [`docs/development/validation.md`](docs/development/validation.md) for the
current commands.

## Review Expectations

Maintainers review correctness, scope, security, compatibility, tests,
documentation, and ownership boundaries. Approval does not waive required
checks. A broad proposal may return to Discussion before implementation.

By contributing, you agree to follow the repository Code of Conduct.
