---
okf_version: "0.2"
type: Function
title: centre_point
description: "Position of the pad's centre `Point`."
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/centre_point
language: rust
---

# centre_point

Position of the pad's centre `Point`.

## Signature

```rust
fn centre_point(sketch: &SketchData, pad: &EditorPad) -> (f64, f64)
```

## Docstring

Position of the pad's centre `Point`.

## Source
Lines 117–129 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
