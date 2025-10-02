# DevOps Guide

This document describes how we work with CI/CD and infrastructure in our projects.

## Merge Request checks behavior
- If a check fails and new commits are added to the MR, all checks should restart automatically.
- If this does not happen, manually rerun the pipeline or ensure it is configured to trigger on new commits.

## General recommendations
- Use `.gitlab-ci.yml` or a similar configuration to automatically run tests and builds.
- Keep all secrets in CI/CD environment variables, not in the repository.
- Before merging into the main branch, make sure all checks have passed.

### GitHub CLI workflow
- The GitHub CLI (`gh`) is pre-installed and authenticated; use it to interact with the hosted repository.
- Confirm the canonical remote with `git remote -v`—`origin` must point to `https://github.com/qqrm/CV.git`.
- Push feature branches with `git push --set-upstream origin <branch>` and open pull requests via `gh pr create`.
- Monitor CI from the terminal using `gh run watch` to follow pipeline progress without leaving the shell.

### Security check
All automated workflows start with a security step. If the author of a pull
request is not `qqrm`, the pipeline stops immediately and no jobs run.

### Workflow permissions and secrets
- Define a `permissions` block in each GitHub Actions workflow.
- Grant only the scopes required by the jobs (for example, `contents: read` or `actions: write`).
- Reference sensitive data through GitHub Secrets and avoid hardcoding credentials.
- Validate and sanitize all external inputs when using `workflow_dispatch` or other triggers.

## Automatic pull request merging
- The `.github/workflows/auto_merge.yml` workflow merges pull requests as soon as all checks succeed.
- Do not remove or disable this workflow. Auto-merge helps keep the main branch healthy.

## Workflow and action naming scheme
Consistent naming keeps automation discoverable and predictable.

- **Workflow files**: use kebab-case filenames that start with a verb describing the outcome. Examples: `build-and-test.yml`, `deploy-preview.yml`.
- **Workflow names**: set the `name` field to the same verb-first phrase in Title Case (for example, `Build and Test`).
- **Jobs and steps**: prefer verb-focused kebab-case identifiers such as `run-tests` or `publish-artifacts` to mirror the workflow intent.
- **Reusable workflows and composite actions**: store them in directories whose names match the workflow/action name in kebab-case, and start their `name` fields with the same verb-first phrase.
- **Published actions**: when exposing actions publicly, keep the repository name kebab-cased and verb-oriented (for example, `prepare-release-action`).


## Local PDF builds
To compile PDFs locally, install the Typst CLI:

```bash
cargo install typst-cli
```

Once installed, build both light and dark variants for each locale:

```bash
typst compile --root . typst/en/Belyakov_en.typ typst/en/Belyakov_en_light.pdf
typst compile --root . typst/en/Belyakov_en_dark.typ typst/en/Belyakov_en_dark.pdf
typst compile --root . typst/ru/Belyakov_ru.typ typst/ru/Belyakov_ru_light.pdf
typst compile --root . typst/ru/Belyakov_ru_dark.typ typst/ru/Belyakov_ru_dark.pdf
```

### Local pipeline runs
CI workflows are defined in GitHub Actions. Use the [`act`](https://github.com/nektos/act) tool to execute them locally.

#### Prerequisites
- Docker Engine installed and running.
- The `act` binary in your `PATH`.

##### Install Docker on Debian/Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y docker.io
sudo dockerd >/tmp/dockerd.log 2>&1 &
```

##### Install act
```bash
curl -sSL https://raw.githubusercontent.com/nektos/act/master/install.sh | \
  sudo bash -s -- -b /usr/local/bin
act --version
# act version 0.2.80
```

#### Running workflows
Run the CI workflow for a pull request from the repository root:
```bash
act pull_request
```

For a dry run that displays the planned steps without executing them:
```bash
act pull_request -n
# *DRYRUN* [ci/ci] 🚀  Start image=catthehacker/ubuntu:act-latest
# Error: Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
```


## Enabling Logs
`sitegen` binaries use [`env_logger`](https://docs.rs/env_logger/) for logging. Set the
`RUST_LOG` environment variable to control verbosity:

```bash
RUST_LOG=info cargo run --bin generate
RUST_LOG=debug cargo run --bin validate
```

The default level is `warn`. Increase to `debug` for more details or
lower to `error` to reduce noise.

## Avatars directory
Role descriptions in Markdown format are stored in the `avatars/` folder at the repository root. Each file describes a typical project role and can be reused in documentation or onboarding materials.

## Documentation guidelines
All Markdown (`.md`) and Mermaid (`.mmd`) files must use **uppercase** filenames. See `DOCS/GUIDELINES.MD` for details.

## Tooling reference
For the list of recommended CLI utilities and installation instructions see `TOOLS/ENVIRONMENT.MD`.
