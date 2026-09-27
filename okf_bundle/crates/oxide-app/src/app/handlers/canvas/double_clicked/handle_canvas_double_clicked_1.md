---
okf_version: "0.2"
type: Function
title: handle_canvas_double_clicked
resource: crates/oxide-app/src/app/handlers/canvas/double_clicked.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/double_clicked/handle_canvas_double_clicked_1
language: rust
---

# handle_canvas_double_clicked

## Signature

```rust
pub(super) fn handle_canvas_double_clicked(
        &mut self,
        world_x: f64,
        world_y: f64,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Source
Lines 7–260 in `crates/oxide-app/src/app/handlers/canvas/double_clicked.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [double_clicked](/crates/oxide-app/src/app/handlers/canvas/double_clicked.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [pre_placement_shape](/crates/oxide-app/src/app/handlers/canvas/mod/pre_placement_shape.md) |
| calls | [expand_char_escapes](/crates/oxide-app/src/schematic_runtime/text/expand_char_escapes.md) |
