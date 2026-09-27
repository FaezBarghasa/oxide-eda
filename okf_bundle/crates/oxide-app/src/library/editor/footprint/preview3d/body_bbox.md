---
okf_version: "0.2"
type: Function
title: body_bbox
resource: crates/oxide-app/src/library/editor/footprint/preview3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/preview3d/body_bbox
language: rust
---

# body_bbox

## Signature

```rust
fn body_bbox(fp: &Footprint) -> (f64, f64, f64, f64)
```

## Source
Lines 289–307 in `crates/oxide-app/src/library/editor/footprint/preview3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview3d](/crates/oxide-app/src/library/editor/footprint/preview3d.md) |
| calls | [compute_bbox](/crates/oxide-app/src/library/editor/footprint/preview3d/compute_bbox.md) |
| called_by | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
