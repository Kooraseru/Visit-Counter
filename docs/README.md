<div align="center">
  <img src="../content/assets/branding/Billboard.svg" alt="Visit-Counter" width="860">
  <h3>A GitHub-native repository view counter, built in Rust and delivered as a Docker action.</h3>

  <p>
    <a href="https://github.com/Kooraseru/Visit-Counter"><img alt="Stars + Issues + License" src="https://shieldcn.dev/group/github/stars/Kooraseru/Visit-Counter+github/Kooraseru/Visit-Counter/issues+github/license/Kooraseru/Visit-Counter.svg?variant=outline"></a>
  </p>

  <img src="https://kooraseru.github.io/Visit-Counter/visit-counter/views.gif" alt="Repository views">

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
- Generates a self-contained, README-safe GIF from customizable digit artwork.
- Runs as a containerized Rust GitHub Action with no external counter server.
- Publishes generated history and counter output to an automation-owned branch.

<h2 id="quick-start">Quick Start</h2>

1. Follow [`src/README.md`](../src/README.md) to copy the self-contained `.github` package into the repository being counted.
2. Create a fine-grained token restricted to that repository with Administration read permission.
3. Store the token as the repository Actions secret `PERSONAL_RELEASE_TOKEN`.
4. Run the workflow once, then embed `https://raw.githubusercontent.com/OWNER/REPOSITORY/visit-counter/views.gif`.

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

The project code, documentation, and bundled artwork are licensed under the [ISC License](../LICENSE).

### Contributors

<a href="https://github.com/Kooraseru/Visit-Counter/graphs/contributors" target="_blank">
  <img src="https://contrib.rocks/image?repo=Kooraseru/Visit-Counter" alt="Visit-Counter contributors">
</a>
