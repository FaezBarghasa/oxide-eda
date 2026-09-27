---
okf_version: "0.2"
type: Function
title: arm_project_rename
description: Open the rename modal targeting a project root.
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/arm_project_rename
language: rust
---

# arm_project_rename

Open the rename modal targeting a project root.

## Signature

```rust
fn arm_project_rename(app: &mut Oxide, target: &Path, new_stem: &str)
```

## Docstring

Open the rename modal targeting a project root.

## Source
Lines 83–91 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [close_rename_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_rename_dialog_dismisses_modal_without_filesystem_changes.md) |
| called_by | [f6_project_rename_does_not_touch_companion_snxsch_snxpcb](/crates/oxide-app/tests/regression/project/f6_project_rename_does_not_touch_companion_snxsch_snxpcb.md) |
| called_by | [f6_project_rename_refuses_to_overwrite_existing_target](/crates/oxide-app/tests/regression/project/f6_project_rename_refuses_to_overwrite_existing_target.md) |
| called_by | [f6_project_rename_rejects_path_separators_in_buffer](/crates/oxide-app/tests/regression/project/f6_project_rename_rejects_path_separators_in_buffer.md) |
| called_by | [f6_project_rename_with_unchanged_stem_is_a_silent_noop](/crates/oxide-app/tests/regression/project/f6_project_rename_with_unchanged_stem_is_a_silent_noop.md) |
| called_by | [project_rename_migrates_dirty_paths_to_new_path](/crates/oxide-app/tests/regression/project/project_rename_migrates_dirty_paths_to_new_path.md) |
| called_by | [rename_buffer_changed_updates_modal_buffer](/crates/oxide-app/tests/regression/project/rename_buffer_changed_updates_modal_buffer.md) |
