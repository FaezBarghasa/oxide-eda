---
okf_version: "0.2"
type: Function
title: create_test_git_repo
description: "Helper: create a git repository with sample commits and semantic version tags."
resource: crates/oxide-library/tests/dependency_manager_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:30:03Z"
concept_id: crates/oxide-library/tests/dependency_manager_tests/create_test_git_repo
language: rust
---

# create_test_git_repo

Helper: create a git repository with sample commits and semantic version tags.

## Signature

```rust
fn create_test_git_repo() -> (TempDir, PathBuf)
```

## Docstring

Helper: create a git repository with sample commits and semantic version tags.

## Source
Lines 10–101 in `crates/oxide-library/tests/dependency_manager_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dependency_manager_tests](/crates/oxide-library/tests/dependency_manager_tests.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| called_by | [test_dependency_exact_tag_and_restore](/crates/oxide-library/tests/dependency_manager_tests/test_dependency_exact_tag_and_restore.md) |
| called_by | [test_dependency_resolution_semver](/crates/oxide-library/tests/dependency_manager_tests/test_dependency_resolution_semver.md) |
