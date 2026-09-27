---
okf_version: "0.2"
type: Function
title: pre_placement_shape
description: Read the shape width + fill defaults out of the current
resource: crates/oxide-app/src/app/handlers/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/mod/pre_placement_shape
language: rust
---

# pre_placement_shape

Read the shape width + fill defaults out of the current

## Signature

```rust
fn pre_placement_shape(
    doc: &super::super::state::DocumentState,
) -> (f64, oxide_types::schematic::FillType)
```

## Docstring

Read the shape width + fill defaults out of the current
pre_placement slot (TAB-configured) so shape tools pick up the
user's Width/Fill edits when committing the next click.

## Source
Lines 22–40 in `crates/oxide-app/src/app/handlers/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/app/handlers/canvas/mod.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [handle_canvas_double_clicked](/crates/oxide-app/src/app/handlers/canvas/double_clicked/handle_canvas_double_clicked.md) |
