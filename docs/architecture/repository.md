# Repository Architecture

The repository separates authored source, project documentation, GitHub
automation, and generated output.

```text
source
├── docs/                 project rules and documentation
├── LICENSE               canonical license
├── CONTRIBUTING.md       contributor workflow
├── content/              assets plus colocated templates and translation catalogs
│   └── releases/         source-only authored GitHub Release records
├── src/ and tests/       project-owned implementation surfaces
├── .github/              source README, translation guide, metadata, and automation
├── .vscode/              shared editor and portable tool templates
└── .generated/           ignored local output
    ├── site/             MkDocs output
    └── repo/<channel>/   publication payloads
```

The `source` branch is canonical. Automation derives `canary`, `beta`, and
`stable` from immutable source commits and records the source commit in each
generated publication manifest.

### Content Layout

```text
content/
├── locales.toml
├── assets/
│   ├── branding/
│   ├── icons/
│   ├── images/
│   ├── diagrams/
│   ├── screenshots/
│   └── media/
├── pages/
│   ├── README.md
│   └── documentation.toml
└── repo/
    ├── shared/
    │   ├── README.template.md
    │   └── repository.toml
    ├── canary/
    ├── beta/
    └── stable/
```

`README.template.md` is authored Markdown whose `.template` segment is removed
during rendering, producing `docs/README.md` in publication payloads. This
convention keeps the source template from presenting itself as the description
of the `shared/` directory.

`content/locales.toml` is the single locale registry and owns locale IDs,
publication state, and fallback order. `.github/TRANSLATORS.md` explains the contributor
workflow without duplicating that registry. Translation catalogs live beside
the templates they describe. `content/pages/` owns
localizable Pages layouts. `content/repo/shared/` owns repository content common
to all publication channels. Channel directories layer overrides and additions
over `shared/`; do not duplicate shared content between them. Assets are grouped
by their durable purpose.

`content/releases/` owns source-only GitHub Release records. Its `README.md`
defines their required structure. Each record uses
`YYYY.MM.N-KIND.release.md`;
the suffix distinguishes structured release input from ordinary Markdown. The
first heading matches the filename's release ID, and the record provides
non-empty Summary, Notable Changes, Issues Addressed, Compatibility, and
Verification sections. Publication prepends the authored record to GitHub's
generated release notes. Release records do not enter generated publication
branches.

The source `.github/README.md`, root `CONTRIBUTING.md`, root `LICENSE`, and
other canonical control files are English-only. The source README explains the
source branch and routes repository roles. Generated repository READMEs are
separate publication inputs under `content/repo/`.

## Canonical Source

Canonical source, tests, project documentation, GitHub configuration, and
generation tooling live on `source`. Change behavior by editing its canonical
input, never by patching generated output.

`canary`, `beta`, and `stable` are automation-owned publication branches. Their
history is disposable. Direct edits and pull requests targeting these branches
are invalid.

<table>
  <tr><th>Surface</th><th>Purpose</th><th>Editable</th></tr>
  <tr><td><code>src/</code></td><td>Project source when the project defines it</td><td>Yes</td></tr>
  <tr><td><code>tests/</code></td><td>Project tests when the project defines them</td><td>Yes</td></tr>
  <tr><td><code>docs/</code></td><td>Project rules and documentation</td><td>Yes</td></tr>
  <tr><td><code>.github/TRANSLATORS.md</code></td><td>English translation workflow</td><td>Yes</td></tr>
  <tr><td><code>content/</code></td><td>Assets plus colocated Pages and repository templates and catalogs</td><td>Yes</td></tr>
  <tr><td><code>.github/</code></td><td>GitHub policy, intake, validation, and publication automation</td><td>Yes, with maintainer review</td></tr>
  <tr><td><code>.vscode/</code></td><td>Shared editor and portable tool templates</td><td>Yes, except <code>settings.json</code></td></tr>
  <tr><td><code>.generated/</code></td><td>Local build and publication output</td><td>No</td></tr>
  <tr><td><code>canary</code></td><td>Generated earliest-consumption repository</td><td>No</td></tr>
  <tr><td><code>beta</code></td><td>Generated testing repository</td><td>No</td></tr>
  <tr><td><code>stable</code></td><td>Generated production repository</td><td>No</td></tr>
</table>

## Release Identity And Channels

A release ID identifies one chronological publication event. It has the form
`YYYY.MM.N-KIND`, where `N` is a single counter shared by every release kind
within the month. The allowed kinds are `regular`, `hotfix`, and `security`.
For example, `2026.08.3-hotfix` is the third release created in August 2026 and
exists to deliver an urgent correction. `regular` covers planned publication,
`hotfix` delivers an urgent non-security correction, and `security` delivers a
security correction through the project security process.

Release IDs always move forward. They are not renamed, promoted, replaced, or
partitioned by kind. The publication branch records destination state and is
not part of release identity:

<table>
  <tr><td><code>source</code></td><td>Canonical authored state</td></tr>
  <tr><td><code>canary</code></td><td>Frequently moving earliest-consumption channel</td></tr>
  <tr><td><code>beta</code></td><td>Deliberate checkpoint for wider testing</td></tr>
  <tr><td><code>stable</code></td><td>Trusted channel for normal production consumption</td></tr>
</table>

User-specific tooling configuration, private planning, editor settings and
state, caches, secrets, and local build output are not repository source and
must not be committed.

## Generated Output

Generators write only persistent deliverables beneath `.generated/site/` and
`.generated/repo/<channel>/`. Generated files must identify or preserve their
source provenance. If generated content is incorrect, fix the canonical source
or generator and regenerate it.

Generated publication branches contain `.github/publication.json`, recording
the channel, version, immutable source commit, and generation time. They exclude
the authored `docs/` tree and retain only `content/assets/` beneath `content/`.
They include `src/` so the generated branch and attached release archive expose
the project source intended for users.
The publication builder does not copy the authored source `.github/README.md`.
It renders English repository content to `docs/README.md` and additional
languages to `docs/README.<locale>.md`; GitHub uses the English file as the
repository landing README when the root has none. Root `LICENSE` publishes
directly where GitHub license detection and release consumers expect it.

## GitHub Special-File Placement

A valid special filename is insufficient by itself. The file must also occupy
a GitHub-supported path and, when GitHub treats it as repository-wide
configuration, exist on the default `stable` branch.

The builder therefore publishes root community and legal files as common user
payload, while `stable` alone receives the selected `.github` control plane:
funding, support, discussion/issue/pull-request templates, Dependabot and
labeler configuration, generated-release-note configuration, and only the
workflows requiring default-branch discovery or manual dispatch. `canary` and
`beta` receive no authored `.github` files. `CODEOWNERS` remains on `source`
because contributor pull requests use `source` as their base branch.

Root `CITATION.cff` publishes in every channel and reaches default `stable`,
allowing GitHub to expose repository citation metadata from its required root
location.

## Automation And Validation

Add a dependency only for an implemented capability with a responsible
maintainer. Pin automation dependencies and third-party GitHub Actions to
reviewed immutable versions. Workflows use least-privilege permissions and must
not expose secrets to untrusted pull-request code.

Non-trivial workflow logic belongs in scripts that run identically locally and
in GitHub Actions. Every source change must run relevant repository validation.
Behavior changes require corresponding tests, and durable contract changes
require corresponding documentation updates.

Do not bypass branch, review, signing, or publication checks; commit secrets,
credentials, private reports, or local configuration; hand-edit generated
branches; or add registries and manifests without an active consumer.

## Localization

Localization is component-oriented. `content/locales.toml` is the sole registry
for locale IDs, publication state, and explicit fallback order. `en-US` is the
fixed default and is not configurable registry metadata. `.github/TRANSLATORS.md`
explains how contributors work with the system without listing the current
registry. Component TOML catalogs live beside the templates they translate
under `content/`. Hierarchical keys provide context, and locale values remain
together at each leaf.

Localizable templates use `{{ l10n:component.context.key }}` at translated
fill-in areas. The renderer replaces each key from adjacent component catalogs.
Every leaf defines `en-US`; other missing translations follow their declared
fallback chain and ultimately use English. Pages derives its entire route and
page structure from `content/pages/`; the authored `docs/` tree is not a Pages
input. Material for MkDocs presents the locale registry through its language
selector. JavaScript replaces the current page content without navigation and
stores the selected locale in a cookie.

MkDocs renders each locale into a temporary site, retains English as the visible
site, and stores other builds beneath `_locales/` as runtime content sources.
The language selector fetches those sources without changing the public route.
Temporary localization, staging, and locale-build trees are removed after the
build, leaving only `.generated/site/`. Publication
materializes the English README at the repository root. Authored templates
and catalogs do not enter publication branches; `content/` retains only public
assets. Additional repository README translations publish as
`docs/README.<locale>.md`. Pages are built separately from the immutable source
commit and use runtime locale payloads without locale-specific public routes.

Shared VS Code infrastructure may include:

```text
.vscode/
├── launch.json           local execution entry points
├── tasks.json            local validation and refresh tasks
├── extensions.json       project extension recommendations
├── project.code-snippets project-owned snippets when useful
└── mcp.json              project-owned, portable MCP servers only
```

`.vscode/settings.json` remains ignored because editor settings are private
user state rather than a contributor contract. Public MCP configuration must
not contain secrets, user-specific paths, or user-specific servers.

Projects may add source domains as they adopt the template. Each durable domain
should have one responsible maintainer in source or documentation; avoid duplicated
contracts and hand-maintained registries.
