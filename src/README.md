# Visit-Counter installation package

This directory is a self-contained package for adding Visit-Counter to a
GitHub repository. Its `.github` directory contains both the scheduled workflow
and the local Docker action used by that workflow.

## Install

1. Copy `src/.github/` from this repository into the root of the repository you
   want to count, merging it with any existing `.github/` directory.
2. Create a fine-grained personal access token restricted to the target
   repository with repository **Administration: Read** permission.
3. Save that token as the repository Actions secret
   `PERSONAL_RELEASE_TOKEN`.
4. Run the **Update visit counter** workflow once from the Actions tab.
5. Add the generated counter to your README:

   ```markdown
   ![Repository views](https://raw.githubusercontent.com/OWNER/REPOSITORY/visit-counter/views.gif)
   ```

Replace `OWNER` and `REPOSITORY` with the target repository coordinates.

## Installed files

```text
.github/
├── actions/
│   └── visit-counter/
│       ├── .dockerignore
│       ├── action.yml
│       ├── Dockerfile
│       ├── assets/
│       │   └── digits/
│       └── rust/
│           ├── Cargo.toml
│           ├── Cargo.lock
│           └── src/
└── workflows/
    └── visit-counter.yml
```

The workflow runs hourly and can also be dispatched manually. It stores the
rolling Traffic API history and generated `views.gif` on the automation-owned
`visit-counter` branch.
