---
okf_version: "0.2"
type: Class
title: ComponentPreviewState
description: Component Preview tab state — one per open row.
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/ComponentPreviewState
language: rust
---

# ComponentPreviewState

Component Preview tab state — one per open row.

## Signature

```rust
pub struct ComponentPreviewState
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Component Preview tab state — one per open row.

Per `v0.9-refactor-2-plan.md` §11: a row is the unit of storage
(DBLib model). The preview surface is read-only for Symbol/Footprint;
the form-shaped tabs (Parameters / Supply / Datasheet / Simulation)
edit `row` in-place and persist via `adapter.update_row(table, row, msg)`.
[derive(Debug)]

## Methods

- `library_path`
- `table`
- `row`
- `symbol`
- `footprint`
- `sim`
- `sim_body`
- `active_tab`
- `params_edit_buf`
- `pin_map_state`
- `dirty`

## Source
Lines 153–198 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
