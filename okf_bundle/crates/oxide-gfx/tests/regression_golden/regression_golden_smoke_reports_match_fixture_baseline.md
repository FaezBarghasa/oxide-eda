---
okf_version: "0.2"
type: Function
title: regression_golden_smoke_reports_match_fixture_baseline
description: "[test]"
resource: crates/oxide-gfx/tests/regression_golden.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-gfx/tests/regression_golden/regression_golden_smoke_reports_match_fixture_baseline
language: rust
---

# regression_golden_smoke_reports_match_fixture_baseline

[test]

## Signature

```rust
fn regression_golden_smoke_reports_match_fixture_baseline()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 372–427 in `crates/oxide-gfx/tests/regression_golden.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [regression_golden](/crates/oxide-gfx/tests/regression_golden.md) |
| calls | [load_golden_fixture](/crates/oxide-gfx/tests/regression_golden/load_golden_fixture.md) |
| calls | [run_line_circle_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_line_circle_smoke_pass.md) |
| calls | [run_arc_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_arc_smoke_pass.md) |
| calls | [run_polygon_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_polygon_smoke_pass.md) |
| calls | [run_text_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_text_smoke_pass.md) |
| calls | [run_grid_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_grid_smoke_pass.md) |
| calls | [run_text_geometry_composite_smoke_pass](/crates/oxide-gfx/src/debug_pass/composite/run_text_geometry_composite_smoke_pass.md) |
| calls | [run_grid_overlay_text_composite_smoke_pass](/crates/oxide-gfx/src/debug_pass/composite/run_grid_overlay_text_composite_smoke_pass.md) |
| calls | [assert_float_eq](/crates/oxide-gfx/tests/regression_golden/assert_float_eq.md) |
