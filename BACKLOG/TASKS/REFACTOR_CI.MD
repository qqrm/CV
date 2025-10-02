# Task: Refactor CI to use declarative pipeline templates

## Description
The current CI configuration is growing unwieldy. Move common jobs and steps into reusable templates, such as composite GitHub Actions, shared `just` recipes, or Nix shells. Add documentation to `/TOOLS/ENVIRONMENT.MD`.

## Related files
- .github/workflows/*
- justfile, Makefile.toml
- default.nix, shell.nix

## Acceptance criteria
- CI is readable and declarative
- Local dev and CI use the same scripts where possible
- Onboarding is one command, as described in ENVIRONMENT.MD

## Status
- [ ] Planned
- [ ] In progress
- [ ] Done
