---
okf_version: "0.2"
type: Function
title: fill_to_rgb
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod/fill_to_rgb
language: rust
---

# fill_to_rgb

## Signature

```rust
fn fill_to_rgb(
    fill: FillType,
    stroke: (f32, f32, f32),
    body_fill: (f32, f32, f32),
) -> Option<(f32, f32, f32)>
```

## Source
Lines 89–103 in `crates/oxide-output/src/svg/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [svg](/crates/oxide-output/src/svg/mod.md) |
| called_by | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
| called_by | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
