---
okf_version: "0.2"
type: Function
title: arm_remove_dialog
description: Open the remove modal for a tree leaf.
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/arm_remove_dialog
language: rust
---

# arm_remove_dialog

Open the remove modal for a tree leaf.

## Signature

```rust
fn arm_remove_dialog(app: &mut Oxide, target: &Path)
```

## Docstring

Open the remove modal for a tree leaf.

## Source
Lines 94–104 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [close_remove_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_remove_dialog_dismisses_modal_without_filesystem_changes.md) |
| called_by | [remove_with_delete_choice_unlinks_the_file](/crates/oxide-app/tests/regression/project/remove_with_delete_choice_unlinks_the_file.md) |
| called_by | [remove_with_exclude_choice_keeps_the_file_on_disk](/crates/oxide-app/tests/regression/project/remove_with_exclude_choice_keeps_the_file_on_disk.md) |
