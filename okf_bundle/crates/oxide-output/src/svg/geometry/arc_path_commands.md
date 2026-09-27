---
okf_version: "0.2"
type: Function
title: arc_path_commands
resource: crates/oxide-output/src/svg/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/geometry/arc_path_commands
language: rust
---

# arc_path_commands

## Signature

```rust
pub(super) fn arc_path_commands(
    start: SvgPoint,
    mid: SvgPoint,
    end: SvgPoint,
) -> Vec<SvgPathCommand>
```

## Visibility

- `pub(super)`

## Source
Lines 54–104 in `crates/oxide-output/src/svg/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-output/src/svg/geometry.md) |
| calls | [circle_from_three_points](/crates/oxide-output/src/svg/geometry/circle_from_three_points.md) |
| calls | [arc_sweep](/crates/oxide-output/src/svg/geometry/arc_sweep.md) |
| calls | [pt](/crates/oxide-output/src/svg/mod/pt.md) |
| called_by | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
| called_by | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
