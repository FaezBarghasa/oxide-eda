---
okf_version: "0.2"
type: Function
title: handle_project_git_commit_done
description: "v0.23 — Handler for [`ProjectMsg::GitCommitDone`]. Clears"
resource: crates/oxide-app/src/app/handlers/document_files/git.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/git/handle_project_git_commit_done_1
language: rust
---

# handle_project_git_commit_done

v0.23 — Handler for [`ProjectMsg::GitCommitDone`]. Clears

## Signature

```rust
pub(crate) fn handle_project_git_commit_done(
        &mut self,
        project_root: std::path::PathBuf,
        rel_path: std::path::PathBuf,
        result: Result<String, String>,
    )
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Handler for [`ProjectMsg::GitCommitDone`]. Clears
the matching `inflight_git_commits` entry and logs the result.

## Source
Lines 140–158 in `crates/oxide-app/src/app/handlers/document_files/git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git](/crates/oxide-app/src/app/handlers/document_files/git.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
