# Development And Validation

Repository validation is owned by `.github/workflows/validate.yml`. Run that
exact workflow locally with [`act`](https://nektosact.com/):

```bash
act push --workflows .github/workflows/validate.yml
```

The workflow invokes these component commands, which remain useful for focused
diagnosis:

```bash
python .github/scripts/validate-repository.py
python .github/scripts/test-localization.py
python .github/scripts/test-publication.py
python .github/scripts/test-publication-manifests.py
python .github/scripts/build-docs.py
bash -n .github/scripts/publish-generated-branch.sh
bash .github/scripts/run-actionlint.sh
```

Run the publication workflow locally with [`act`](https://nektosact.com/):

```bash
act workflow_dispatch \
  --workflows .github/workflows/publish.yml \
  --job publish \
  --input channel=stable \
  --input version=2026.08.2-hotfix
```

This executes `.github/workflows/publish.yml`, not a separate local publication
procedure. Select the desired channel and an authored release ID. The workflow
runs the same validation and payload-build steps locally. Steps requiring a
protected GitHub environment, signing credentials, remote branch mutation,
GitHub Release creation, or workflow dispatch report that they are unavailable
and skip cleanly. Local runs build the current workspace and record HEAD as
provenance; GitHub publication continues to require committed canonical source.

The validation workflow checks GitHub configuration, issue form structure,
workflow security invariants, local Markdown links, project residue, and
localization and publication boundary tests. Localization rendering validates
the locale manifest, component trees, template keys, fallback graph, and
default English coverage. An external link check runs separately because it
requires network access.

Projects add formatting, linting, static analysis, unit tests, integration
tests, and build checks only when the corresponding source exists. Every added
check must have a GitHub Actions stage and an identical local command.
