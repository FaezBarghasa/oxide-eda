---
okf_version: "0.2"
type: Function
title: set_mode
resource: crates/oxide-app/src/library/editor/footprint/updates/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/view/set_mode
language: rust
---

# set_mode

## Signature

```rust
fn set_mode(
    editor: &mut crate::app::FootprintEditorState,
    mode: crate::library::editor::footprint::state::EditorMode,
)
```

## Source
Lines 50–94 in `crates/oxide-app/src/library/editor/footprint/updates/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/updates/view.md) |
| calls | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/view/apply.md) |
