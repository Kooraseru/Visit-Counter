<div align="center">
  <img src="../content/assets/branding/Billboard.svg" alt="Visit-Counter" width="860">
  <h3>Rust と Docker Action で構築された、GitHub ネイティブのリポジトリ閲覧カウンター。</h3>

  <p>
    <a href="https://github.com/Kooraseru/Visit-Counter"><img alt="Stars + Issues + License" src="https://shieldcn.dev/group/github/stars/Kooraseru/Visit-Counter+github/Kooraseru/Visit-Counter/issues+github/license/Kooraseru/Visit-Counter.svg?variant=outline"></a>
  </p>

  <img src="https://kooraseru.github.io/Visit-Counter/visit-counter/views.gif" alt="Repository views">

  <table>
    <tr>
      <td align="center"><a href="#features">機能</a></td>
      <td align="center"><a href="#quick-start">クイックスタート</a></td>
      <td align="center"><a href="#documentation">ドキュメント</a></td>
      <td align="center"><a href="#project">プロジェクト</a></td>
    </tr>
  </table>

  <table>
  <tr>
    <td align="center"><a href="README.md">English</a></td>
    <td align="center"><a href="README.ja-JP.md">日本語</a></td>
  </tr>
</table>
</div>

<h2 id="features">機能</h2>

- サードパーティの訪問追跡ではなく、GitHub 自身のリポジトリ Traffic API を使用します。
- GitHub の重複するトラフィック期間を二重計上せず、日ごとの閲覧履歴を保持します。
- カスタマイズ可能な数字画像から、README で安全に使える自己完結型 GIF を生成します。
- 外部カウンターサーバーを使わず、コンテナ化された Rust GitHub Action として動作します。
- 生成された履歴とカウンターをオートメーション管理のブランチへ公開します。

<h2 id="quick-start">クイックスタート</h2>

1. [`src/README.md`](../src/README.md) の手順に従い、自己完結型の `.github` パッケージを計測対象リポジトリへコピーします。
2. 対象リポジトリの Administration 読み取り権限を持つ fine-grained token を作成します。
3. トークンをリポジトリの Actions secret `PERSONAL_RELEASE_TOKEN` として保存します。
4. ワークフローを一度実行し、`https://raw.githubusercontent.com/OWNER/REPOSITORY/visit-counter/views.gif` を埋め込みます。

<h2 id="documentation">ドキュメント</h2>

<table>
  <tr><td><a href="../CONTRIBUTING.md"><code>CONTRIBUTING.md</code></a></td><td>コントリビューターのワークフローを定義します。</td></tr>
  <tr><td><a href="../SECURITY.md"><code>SECURITY.md</code></a></td><td>脆弱性を非公開で報告する方法を定義します。</td></tr>
  <tr><td><a href="https://kooraseru.github.io/Visit-Counter/">公開ドキュメント</a></td><td>生成されたドキュメントサイトを提供します。</td></tr>
</table>

<h2 id="project">プロジェクト</h2>

Visit-Counter は GitHub ネイティブ Action として管理されています。正規の開発は `source` で行い、検証済みの利用者向けリリースは `stable` に生成されます。

### ブランチ

<table>
  <tr><td><code>source</code></td><td>正規の作成ブランチ</td></tr>
  <tr><td><code>canary</code></td><td>生成された早期利用チャンネル</td></tr>
  <tr><td><code>beta</code></td><td>生成されたテストチャンネル</td></tr>
  <tr><td><code>stable</code></td><td>生成された本番チャンネル</td></tr>
</table>

コントリビューターは `source` で変更を作成し、プルリクエストの対象も `source` にします。生成ブランチはオートメーションが管理します。

### ライセンス

プロジェクトのコード、ドキュメント、および同梱アートワークには [ISC License](../LICENSE) が適用されます。

### コントリビューター

<a href="https://github.com/Kooraseru/Visit-Counter/graphs/contributors" target="_blank">
  <img src="https://contrib.rocks/image?repo=Kooraseru/Visit-Counter" alt="Visit-Counter contributors">
</a>
