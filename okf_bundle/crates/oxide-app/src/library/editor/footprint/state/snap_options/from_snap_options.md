---
okf_version: "0.2"
type: Function
title: from_snap_options
description: "Seed the implicit \"Global Snap Grid\" row from a `SnapOptions`"
resource: crates/oxide-app/src/library/editor/footprint/state/snap_options.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/snap_options/from_snap_options
language: rust
---

# from_snap_options

Seed the implicit "Global Snap Grid" row from a `SnapOptions`

## Signature

```rust
impl GridDef { pub fn from_snap_options(opts: &SnapOptions) -> Self }
```

## Visibility

- `pub`

## Docstring

Seed the implicit "Global Snap Grid" row from a `SnapOptions`
snapshot. Used when the FootprintEditorState first materialises
to keep the legacy single-grid behaviour intact.

## Source
Lines 130–138 in `crates/oxide-app/src/library/editor/footprint/state/snap_options.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap_options](/crates/oxide-app/src/library/editor/footprint/state/snap_options.md) |
