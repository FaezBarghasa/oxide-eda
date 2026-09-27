---
okf_version: "0.2"
type: Module
title: project
description: "Project-scoped git adapter — local version control for `.snxprj`"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project
language: rust
---

# project

Project-scoped git adapter — local version control for `.snxprj`

## Docstring

Project-scoped git adapter — local version control for `.snxprj`
and its sibling design files (`.snxsch`, `.snxpcb`, `.snxmat`,
`.snxnet`, `.snxbom`, `.snxout`).

Sister module to [`local_git::LocalGitAdapter`] (which manages the
`.snxlib` directory). This adapter operates at *project-root*
scope — the parent directory of the `.snxprj` file — so the same
repo covers schematic, PCB, simulation models, and exported
artefacts.

```text
my-project/                          (project_root — git working tree)
├── my-project.snxprj                (manifest)
├── sheets/<name>.snxsch
├── pcb/<name>.snxpcb
├── models/                          (3D models, optionally LFS-tracked)
├── outputs/                         (.snxout, generated)
├── .gitattributes                   (lf for .snx*, binary for .step etc)
└── .git/
```

Per-file commit semantics: every save dispatches a single
`commit_path(rel_path, message)` after the atomic write succeeds.
Failure surfaces as a non-modal status-bar warning — the user's
data is on disk regardless of whether git captures it.

Public surface (mirrors the v0.22 PROJECT_GIT_PLAN.md spec):
- [`LocalGitProjectAdapter::open_or_init`]
- [`LocalGitProjectAdapter::commit_path`]
- [`LocalGitProjectAdapter::commit_external_change`]
- [`LocalGitProjectAdapter::file_history`]
- [`LocalGitProjectAdapter::restore_at`]

Concurrency: per-instance `Mutex` serialises every git operation
that mutates `.git/index`. Mirrors the
[`local_git::LocalGitAdapter`] HI-11 fix.

## Relationships

| Type | Target |
|------|--------|
| related | [LocalGitProjectAdapter](/crates/oxide-library/src/adapters/local_git/project/LocalGitProjectAdapter.md) |
| related | [open_or_init](/crates/oxide-library/src/adapters/local_git/project/open_or_init.md) |
| related | [commit_path](/crates/oxide-library/src/adapters/local_git/project/commit_path.md) |
| related | [commit_external_change](/crates/oxide-library/src/adapters/local_git/project/commit_external_change.md) |
| related | [file_history](/crates/oxide-library/src/adapters/local_git/project/file_history.md) |
| related | [restore_at](/crates/oxide-library/src/adapters/local_git/project/restore_at.md) |
| related | [project_root](/crates/oxide-library/src/adapters/local_git/project/project_root.md) |
| related | [restore_at_from_sha](/crates/oxide-library/src/adapters/local_git/project/restore_at_from_sha.md) |
| related | [write_gitattributes](/crates/oxide-library/src/adapters/local_git/project/write_gitattributes.md) |
| related | [open_or_init](/crates/oxide-library/src/adapters/local_git/project/open_or_init.md) |
| related | [commit_path](/crates/oxide-library/src/adapters/local_git/project/commit_path.md) |
| related | [commit_external_change](/crates/oxide-library/src/adapters/local_git/project/commit_external_change.md) |
| related | [file_history](/crates/oxide-library/src/adapters/local_git/project/file_history.md) |
| related | [restore_at](/crates/oxide-library/src/adapters/local_git/project/restore_at.md) |
| related | [project_root](/crates/oxide-library/src/adapters/local_git/project/project_root.md) |
| related | [restore_at_from_sha](/crates/oxide-library/src/adapters/local_git/project/restore_at_from_sha.md) |
| related | [write_gitattributes](/crates/oxide-library/src/adapters/local_git/project/write_gitattributes.md) |
| related | [CommitPathStats](/crates/oxide-library/src/adapters/local_git/project/CommitPathStats.md) |
| related | [commit_diff_stats_for_path](/crates/oxide-library/src/adapters/local_git/project/commit_diff_stats_for_path.md) |
| related | [history_entry_from_commit](/crates/oxide-library/src/adapters/local_git/project/history_entry_from_commit.md) |
| related | [identity_for_repo](/crates/oxide-library/src/adapters/local_git/project/identity_for_repo.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
