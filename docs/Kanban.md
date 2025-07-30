# Task Management with Git

This repository organizes work items using a simple directory based Kanban board.

## Directory structure

- `tasks/backlog/` – new tasks are created here as individual Markdown files.
- `tasks/in-progress/` – move a task file here when work on it begins.
- `tasks/done/` – completed tasks are archived in this directory.

## Recommended workflow

1. Create a new Markdown file in `tasks/backlog/` describing the task.
2. When you start working on it, move the file to `tasks/in-progress/`.
3. After finishing the work, move the file to `tasks/done/`.

Following this structure keeps tasks visible and makes progress easy to track.

## Tracking subtasks

Large tasks can be broken down into subtasks inside the Markdown file stored in
`tasks/in-progress/`. Use a checklist so that each small step is visible. Each
merge request may implement one subtask, allowing incremental delivery and
focused reviews. Team members are free to switch roles—for example from analyst
to engineer to tester—as the work progresses. Apply automation and thorough
testing whenever possible.
