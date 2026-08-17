# Project Documentation

This directory README contains repository rules and project documentation. Start with
the repository architecture for source and generation boundaries, then use the
remaining sections for implementation details.

- [Repository architecture](architecture/repository.md)
- [Development and validation](development/validation.md)
- [Documentation standards](standards/documentation.md)
- [Configuration standards](standards/configuration.md)
=======
  <img src="https://kooraseru.github.io/Visit-Counter/visit-counter/views.svg" alt="Repository views">

  <table>
    <tr>
      <td align="center"><a href="#features">Features</a></td>
      <td align="center"><a href="#quick-start">Quick Start</a></td>
      <td align="center"><a href="#documentation">Documentation</a></td>
      <td align="center"><a href="#project">Project</a></td>
    </tr>
  </table>

  <table>
  <tr>
    <td align="center"><a href="README.md">English</a></td>
    <td align="center"><a href="README.ja-JP.md">日本語</a></td>
  </tr>
</table>
</div>

<h2 id="features">Features</h2>

- Uses GitHub's own repository Traffic API instead of third-party visitor tracking.
- Preserves daily view history without double-counting GitHub's overlapping traffic window.
- Generates a self-contained SVG from customizable digit artwork.
- Runs as a containerized Rust GitHub Action with no external counter server.
- Publishes generated history and counter output to an automation-owned branch.

<h2 id="quick-start">Quick Start</h2>

1. Copy [`examples/update-visit-counter.yml`](../examples/update-visit-counter.yml) to `.github/workflows/update-visit-counter.yml` in the repository being counted.
2. Create a fine-grained token restricted to that repository with Administration read permission.
3. Store the token as the repository Actions secret `TRAFFIC_TOKEN`.
4. Enable GitHub Pages for the repository and run the workflow once. Embed `https://OWNER.github.io/REPOSITORY/visit-counter/views.svg`. The workflow publishes `views.svg` and its GIF digit assets together, so animated digits work without the restrictive `raw.githubusercontent.com` CSP.

<h2 id="documentation">Documentation</h2>

<table>
  <tr><td><a href="../CONTRIBUTING.md"><code>CONTRIBUTING.md</code></a></td><td>Defines the contributor workflow.</td></tr>
  <tr><td><a href="../SECURITY.md"><code>SECURITY.md</code></a></td><td>Defines private vulnerability reporting.</td></tr>
  <tr><td><a href="https://kooraseru.github.io/Visit-Counter/">Published documentation</a></td><td>Provides the generated documentation site.</td></tr>
</table>

<h2 id="project">Project</h2>

Visit-Counter is maintained as a GitHub-native action. Canonical development happens on `source`; tested consumer releases are generated on `stable`.

### Branches

<table>
  <tr><td><code>source</code></td><td>Canonical authoring branch</td></tr>
  <tr><td><code>canary</code></td><td>Generated earliest-consumption channel</td></tr>
  <tr><td><code>beta</code></td><td>Generated testing channel</td></tr>
  <tr><td><code>stable</code></td><td>Generated production channel</td></tr>
</table>

Contributors author changes and target pull requests at `source`. Automation owns the generated branches.

### License

Licensed under the [Observer License 0.1](https://github.com/Kooraseru/Observer-License).

### Contributors

<a href="https://github.com/Kooraseru/Visit-Counter/graphs/contributors" target="_blank">
  <img src="https://contrib.rocks/image?repo=Kooraseru/Visit-Counter" alt="Visit-Counter contributors">
</a>

## Visit counter on GitHub Pages

Enable GitHub Pages for the repository and run the visit-counter workflow. Embed
`https://OWNER.github.io/REPOSITORY/visit-counter/views.svg`. The workflow publishes
the SVG and its GIF digit assets together, avoiding the restrictive
`raw.githubusercontent.com` CSP.
