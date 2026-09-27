---
okf_version: "0.2"
type: Function
title: draw_text_note_preview
resource: crates/oxide-app/src/schematic_runtime/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/text/draw_text_note_preview
language: rust
---

# draw_text_note_preview

## Signature

```rust
pub fn draw_text_note_preview(
    frame: &mut canvas::Frame,
    note: &TextNote,
    transform: &ScreenTransform,
    color: Color,
)
```

## Visibility

- `pub`

## Source
Lines 11–52 in `crates/oxide-app/src/schematic_runtime/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-app/src/schematic_runtime/text.md) |
| calls | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [draw_ghost_text](/crates/oxide-app/src/canvas/draw/ghosts/draw_ghost_text.md) |
