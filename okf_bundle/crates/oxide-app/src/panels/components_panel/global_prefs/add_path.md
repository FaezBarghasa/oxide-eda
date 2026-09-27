---
okf_version: "0.2"
type: Function
title: add_path
description: Append a path to the global list and persist. Idempotent — already-
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/add_path
language: rust
---

# add_path

Append a path to the global list and persist. Idempotent — already-

## Signature

```rust
pub fn add_path(path: PathBuf) -> Result<Vec<GlobalLibraryEntry>, String>
```

## Visibility

- `pub`

## Docstring

Append a path to the global list and persist. Idempotent — already-
present paths are skipped. Returns the resulting full list.

## Source
Lines 127–138 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |
| calls | [save](/crates/oxide-app/src/panels/components_panel/global_prefs/save.md) |
| called_by | [handle_components_panel_add_library_at](/crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_add_library_at.md) |
| called_by | [handle_components_panel_promote_to_global](/crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_promote_to_global.md) |
| called_by | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| called_by | [commit_path](/crates/oxide-library/src/adapters/local_git/primitives/commit_path.md) |
| called_by | [commit_path](/crates/oxide-library/src/adapters/local_git/project/commit_path.md) |
| called_by | [create_test_git_repo](/crates/oxide-library/tests/dependency_manager_tests/create_test_git_repo.md) |
| called_by | [commit_file](/crates/oxide-library/tests/project_file_history/commit_file.md) |
