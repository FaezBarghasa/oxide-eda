---
okf_version: "0.2"
type: Function
title: close_remove_dialog_dismisses_modal_without_filesystem_changes
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
concept_id: crates/oxide-app/tests/regression/project/close_remove_dialog_dismisses_modal_without_filesystem_changes
language: rust
---

# close_remove_dialog_dismisses_modal_without_filesystem_changes

[test]

## Signature

```rust
fn close_remove_dialog_dismisses_modal_without_filesystem_changes()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 534–544 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| calls | [fixture_project_with_companions](/crates/oxide-app/tests/regression/project/fixture_project_with_companions.md) |
| calls | [arm_remove_dialog](/crates/oxide-app/tests/regression/project/arm_remove_dialog.md) |
