---
okf_version: "0.2"
type: Function
title: handle_canvas_clicked
resource: crates/oxide-app/src/app/handlers/canvas/clicked.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked
language: rust
---

# handle_canvas_clicked

## Signature

```rust
impl Oxide { pub(super) fn handle_canvas_clicked(&mut self, world_x: f64, world_y: f64) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 8–659 in `crates/oxide-app/src/app/handlers/canvas/clicked.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clicked](/crates/oxide-app/src/app/handlers/canvas/clicked.md) |
| calls | [hit_test_polygon](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_polygon.md) |
| calls | [passes_filter](/crates/oxide-app/src/app/handlers/selection_workflow/passes_filter.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [escape_for_standard](/crates/oxide-app/src/schematic_runtime/text/escape_for_standard.md) |
| calls | [Label](/crates/oxide-types/src/schematic/sheet/Label.md) |
| calls | [TextNote](/crates/oxide-types/src/schematic/sheet/TextNote.md) |
| calls | [constrain_segments](/crates/oxide-app/src/app/helpers/constrain_segments.md) |
| calls | [pre_placement_shape](/crates/oxide-app/src/app/handlers/canvas/mod/pre_placement_shape.md) |
