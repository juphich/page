---
name: epic-workflow
description: Manage PAGE repository backlogs, epics, tasks, and implementation work using the required documents under docs. Use when planning, documenting, implementing, or reviewing repository work.
---

# PAGE Epic Workflow

Apply the project rules in [../../rules/project.md](../../rules/project.md) before starting repository work.

## Backlog Workflow

1. Store each backlog in its own `docs/backlogs/백로그_<제목>.md` file.
2. Define only one coherent goal per backlog using [references/templates.md](references/templates.md).
3. Count all physical lines, including blank lines, after creating or updating the file.
4. If the file would exceed 50 lines or combines multiple goals, warn that it is likely too broad and propose a concrete split instead of extending it unchanged.
5. When backlog work starts, connect it to an epic and task; a backlog does not replace either document.

## Epic Workflow

1. Identify the epic and task that contain the requested work.
2. Assign every task a `pt-xxxx` code using four zero-padded digits.
3. Inspect the epic's `index.md`, `plan.md`, optional `design.md`, and relevant task document before changing project files.
4. If the work has no documents, create the epic and task documents first using [references/templates.md](references/templates.md).
5. Keep the task checklist in `plan.md` synchronized with actual progress.
6. Record material decisions, changed files, and verification results in the task document.
7. Mark a task complete only after its stated completion conditions and checks pass.
8. When the epic ends, update its final status and move the entire epic directory from `docs/epics/` to `docs/archive/` without renaming or restructuring it.

## Design Routing

- Keep only product direction, shared architecture, and cross-epic invariants in `docs/design.md`.
- Put design shared by tasks in one epic in that epic's `design.md`.
- Put design limited to one task in its `pt-xxxx` document.
- Move misplaced details to the narrowest applicable document and link to them instead of duplicating them.

Do not create work documents outside `docs/`. Do not silently assign ambiguous work to an unrelated epic; ask for the epic or propose a new one when the choice materially affects scope.
