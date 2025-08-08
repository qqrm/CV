# Task: Add role-based CV configurations

## Description
Enable generating tailored CV versions for different positions by storing role-specific settings, such as which sections to include and emphasis, in YAML or JSON files.

## Steps
- Design a configuration schema for roles.
- Implement parsing of role configuration files.
- Integrate role settings into the CV generation pipeline.

## Acceptance Criteria
- Role configuration files exist for each supported position.
- CV generation uses role settings to include the correct sections.
- `cargo test` and Typst builds pass.
