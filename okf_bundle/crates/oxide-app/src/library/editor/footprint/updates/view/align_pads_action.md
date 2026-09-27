---
okf_version: "0.2"
type: Function
title: align_pads_action
resource: crates/oxide-app/src/library/editor/footprint/updates/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/view/align_pads_action
language: rust
---

# align_pads_action

## Signature

```rust
fn align_pads_action(
    editor: &mut crate::app::FootprintEditorState,
    op: crate::library::editor::footprint::state::AlignOp,
)
```

## Source
Lines 219–269 in `crates/oxide-app/src/library/editor/footprint/updates/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/updates/view.md) |
| calls | [align_pads](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_pads.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/view/apply.md) |
