---
okf_version: "0.2"
type: Function
title: align_confirm
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm
language: rust
---

# align_confirm

## Signature

```rust
fn align_confirm(editor: &mut crate::app::FootprintEditorState)
```

## Source
Lines 293–369 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| calls | [align_pads](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_pads.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
