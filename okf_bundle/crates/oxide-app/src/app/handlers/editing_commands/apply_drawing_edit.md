---
okf_version: "0.2"
type: Function
title: apply_drawing_edit
description: "Patch a `SchDrawing` with a single field edit. Mutates a cloned"
resource: crates/oxide-app/src/app/handlers/editing_commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/editing_commands/apply_drawing_edit
language: rust
---

# apply_drawing_edit

Patch a `SchDrawing` with a single field edit. Mutates a cloned

## Signature

```rust
fn apply_drawing_edit(
    current: oxide_types::schematic::SchDrawing,
    edit: crate::app::contracts::DrawingFieldEdit,
) -> Option<oxide_types::schematic::SchDrawing>
```

## Docstring

Patch a `SchDrawing` with a single field edit. Mutates a cloned
copy in place to avoid rebuilding every SchDrawing variant — and
to preserve every future field (stroke_color et al) automatically.
Returns `None` when the edit is incompatible with the drawing
variant (e.g. `ArcRadius` on a Rect). Arc edits convert the
Altium-style (center, radius, start/end angle) fields back to
Standard's stored (start, mid, end) triple.

## Source
Lines 198–325 in `crates/oxide-app/src/app/handlers/editing_commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editing_commands](/crates/oxide-app/src/app/handlers/editing_commands.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [normalize_rad](/crates/oxide-app/src/app/handlers/editing_commands/normalize_rad.md) |
| calls | [norm_ccw](/crates/oxide-app/src/app/handlers/editing_commands/norm_ccw.md) |
| called_by | [handle_update_drawing_field](/crates/oxide-app/src/app/handlers/editing_commands/handle_update_drawing_field.md) |
