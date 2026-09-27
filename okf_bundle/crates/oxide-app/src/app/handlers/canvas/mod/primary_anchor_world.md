---
okf_version: "0.2"
type: Function
title: primary_anchor_world
description: "Resolve a selected item's primary anchor — the world point that should"
resource: crates/oxide-app/src/app/handlers/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/mod/primary_anchor_world
language: rust
---

# primary_anchor_world

Resolve a selected item's primary anchor — the world point that should

## Signature

```rust
fn primary_anchor_world(
    snap: &crate::schematic_runtime::SchematicRenderSnapshot,
    item: &oxide_types::schematic::SelectedItem,
) -> Option<(f64, f64)>
```

## Docstring

Resolve a selected item's primary anchor — the world point that should
snap to the grid (connection point for labels/wires/symbols, etc.).

## Source
Lines 349–412 in `crates/oxide-app/src/app/handlers/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/app/handlers/canvas/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [handle_canvas_interaction_event](/crates/oxide-app/src/app/handlers/canvas/mod/handle_canvas_interaction_event.md) |
