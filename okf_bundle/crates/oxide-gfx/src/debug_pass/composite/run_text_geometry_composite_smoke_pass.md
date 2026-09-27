---
okf_version: "0.2"
type: Function
title: run_text_geometry_composite_smoke_pass
resource: crates/oxide-gfx/src/debug_pass/composite.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/debug_pass/composite/run_text_geometry_composite_smoke_pass
language: rust
---

# run_text_geometry_composite_smoke_pass

## Signature

```rust
pub fn run_text_geometry_composite_smoke_pass() -> Result<CompositeSmokeReport, String>
```

## Visibility

- `pub`

## Source
Lines 119–140 in `crates/oxide-gfx/src/debug_pass/composite.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [composite](/crates/oxide-gfx/src/debug_pass/composite.md) |
| calls | [run_text_geometry_composite_smoke_pass_with](/crates/oxide-gfx/src/debug_pass/composite/run_text_geometry_composite_smoke_pass_with.md) |
| called_by | [text_compositing_order_places_text_above_geometry](/crates/oxide-gfx/src/debug_pass/tests/text_compositing_order_places_text_above_geometry.md) |
| called_by | [regression_golden_smoke_reports_match_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_smoke_reports_match_fixture_baseline.md) |
