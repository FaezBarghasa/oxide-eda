---
okf_version: "0.2"
type: Function
title: from_footprint
description: "Build canvas state from the primitive's pad list."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/from_footprint
language: rust
---

# from_footprint

Build canvas state from the primitive's pad list.

## Signature

```rust
impl FootprintEditorState { pub fn from_footprint(fp: &Footprint) -> Self }
```

## Visibility

- `pub`

## Docstring

Build canvas state from the primitive's pad list.

## Source
Lines 263–271 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| calls | [relink_pads_to_sketch](/crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch.md) |
