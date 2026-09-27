---
okf_version: "0.2"
type: Function
title: scale_axis
description: "Expand (`expand=true`) or contract the centre-to-centre gaps of"
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/scale_axis
language: rust
---

# scale_axis

Expand (`expand=true`) or contract the centre-to-centre gaps of

## Signature

```rust
fn scale_axis(centres: &mut [(f64, f64)], axis: SpacingAxis, pivot: f64, step: f64, expand: bool)
```

## Docstring

Expand (`expand=true`) or contract the centre-to-centre gaps of
`centres` along `axis`, pivoting about `pivot`. Every gap changes by
`step`, so the outermost span changes by `step*(n-1)`; relative
spacing is preserved by scaling each offset from the pivot by
`new_span / old_span`. Contract is clamped so the span never goes
negative. No-op when all centres are coincident on that axis.

## Source
Lines 147–173 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| called_by | [apply_align](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_align.md) |
