---
okf_version: "0.2"
type: Function
title: commit_save_to_project_git
description: v0.22 Phase 8.4 — auto-commit a saved file into the owning
resource: crates/oxide-app/src/app/handlers/document_files/git.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/git/commit_save_to_project_git_1
language: rust
---

# commit_save_to_project_git

v0.22 Phase 8.4 — auto-commit a saved file into the owning

## Signature

```rust
pub fn commit_save_to_project_git(
        &mut self,
        file_path: &std::path::Path,
        default_message: &str,
    )
```

## Visibility

- `pub`

## Docstring

v0.22 Phase 8.4 — auto-commit a saved file into the owning
project's local Git repo when `enable_git` is on.

Walks `document_state.projects` looking for the project whose
`data.dir` is a prefix of `file_path`. If found AND
`data.enable_git == true`, opens
[`LocalGitProjectAdapter`] and runs `commit_path`.

Failure is best-effort: logged + surfaced as a non-modal
status warning, never blocks the save. The user's data is on
disk regardless of whether git captures it.

v0.23 — Async pipeline. The save-handler synchronously
resolves the owning project + relative path (cheap — just
walks `DocumentState.projects`), then pushes a
[`crate::app::state::PendingGitCommit`] onto
`pending_git_commits` and adds the pair to
`inflight_git_commits` so the status bar's "Saving…" pill
shows immediately. The actual `git2` work runs in
`finish_update`'s [`Self::drain_pending_git_commits`] which
emits one `Task::perform` per queued commit. Result lands as
`Message::Project(ProjectMsg::GitCommitDone)`; the handler clears
the inflight entry.

## Source
Lines 31–67 in `crates/oxide-app/src/app/handlers/document_files/git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git](/crates/oxide-app/src/app/handlers/document_files/git.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
