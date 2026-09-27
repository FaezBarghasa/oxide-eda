---
okf_version: "0.2"
type: Function
title: editor_with_pads
description: "A wrapper editor pre-seeded with `n` pads at distinct positions and"
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/editor_with_pads
language: rust
---

# editor_with_pads

A wrapper editor pre-seeded with `n` pads at distinct positions and

## Signature

```rust
fn editor_with_pads(n: usize) -> crate::app::FootprintEditorState
```

## Docstring

A wrapper editor pre-seeded with `n` pads at distinct positions and
reset to a clean state, so a test asserts the *dispatcher* — not the
fixture — is what dirties the document / stacks history.

## Source
Lines 270–278 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [default_editor](/crates/oxide-app/src/library/editor/footprint/tests/default_editor.md) |
| called_by | [issue_146_align_to_grid_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_align_to_grid_with_no_selection_stays_clean.md) |
| called_by | [issue_146_context_click_on_new_pad_clears_stale_extras](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_click_on_new_pad_clears_stale_extras.md) |
| called_by | [issue_146_context_select_all_fills_extras_like_active_bar](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_select_all_fills_extras_like_active_bar.md) |
| called_by | [issue_146_rotate_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_no_selection_stays_clean.md) |
| called_by | [issue_146_rotate_with_selection_dirties_and_snapshots_once](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_selection_dirties_and_snapshots_once.md) |
