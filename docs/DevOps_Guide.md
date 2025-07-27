# DevOps Guide

This document describes how we work with CI/CD and infrastructure in our projects.

## Merge Request checks behavior
- If a check fails and new commits are added to the MR, all checks should restart automatically.
- If this does not happen, manually rerun the pipeline or ensure it is configured to trigger on new commits.

## General recommendations
- Use `.gitlab-ci.yml` or a similar configuration to automatically run tests and builds.
- Keep all secrets in CI/CD environment variables, not in the repository.
- Before merging into the main branch, make sure all checks have passed.

## Automatic pull request merging
- The `.github/workflows/auto_merge.yml` workflow merges pull requests as soon as all checks succeed.
- Do not remove or disable this workflow. Auto-merge helps keep the main branch healthy.

