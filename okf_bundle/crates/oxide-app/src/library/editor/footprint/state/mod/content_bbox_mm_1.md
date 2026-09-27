---
okf_version: "0.2"
type: Function
title: content_bbox_mm
description: Bounding box of the entire footprint (pads + courtyard) in mm.
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/content_bbox_mm_1
language: rust
---

# content_bbox_mm

Bounding box of the entire footprint (pads + courtyard) in mm.

## Signature

```rust
pub fn content_bbox_mm(&self) -> Option<(f64, f64, f64, f64)>
```

## Visibility

- `pub`

## Docstring

Bounding box of the entire footprint (pads + courtyard) in mm.

## Source
Lines 343–359 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
