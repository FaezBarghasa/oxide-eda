---
okf_version: "0.2"
type: Function
title: project_rename_migrates_dirty_paths_to_new_path
description: "[test]"
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/project_rename_migrates_dirty_paths_to_new_path
language: rust
---

# project_rename_migrates_dirty_paths_to_new_path

[test]

## Signature

```rust
fn project_rename_migrates_dirty_paths_to_new_path()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 379–397 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| calls | [fixture_project_with_companions](/crates/oxide-app/tests/regression/project/fixture_project_with_companions.md) |
| calls | [arm_project_rename](/crates/oxide-app/tests/regression/project/arm_project_rename.md) |
