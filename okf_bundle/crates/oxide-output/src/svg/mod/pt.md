---
okf_version: "0.2"
type: Function
title: pt
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod/pt
language: rust
---

# pt

## Signature

```rust
fn pt(x: f32, y: f32) -> SvgPoint
```

## Source
Lines 140–142 in `crates/oxide-output/src/svg/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [svg](/crates/oxide-output/src/svg/mod.md) |
| called_by | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
| called_by | [arc_path_commands](/crates/oxide-output/src/svg/geometry/arc_path_commands.md) |
| called_by | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
