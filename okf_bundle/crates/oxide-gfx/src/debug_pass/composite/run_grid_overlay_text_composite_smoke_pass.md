---
okf_version: "0.2"
type: Function
title: run_grid_overlay_text_composite_smoke_pass
resource: crates/oxide-gfx/src/debug_pass/composite.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/debug_pass/composite/run_grid_overlay_text_composite_smoke_pass
language: rust
---

# run_grid_overlay_text_composite_smoke_pass

## Signature

```rust
pub fn run_grid_overlay_text_composite_smoke_pass() -> Result<OverlayCompositeSmokeReport, String>
```

## Visibility

- `pub`

## Source
Lines 284–328 in `crates/oxide-gfx/src/debug_pass/composite.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [composite](/crates/oxide-gfx/src/debug_pass/composite.md) |
| calls | [run_grid_overlay_text_composite_smoke_pass_with](/crates/oxide-gfx/src/debug_pass/composite/run_grid_overlay_text_composite_smoke_pass_with.md) |
| called_by | [overlay_compositing_order_places_overlay_between_geometry_and_text](/crates/oxide-gfx/src/debug_pass/tests/overlay_compositing_order_places_overlay_between_geometry_and_text.md) |
| called_by | [regression_golden_smoke_reports_match_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_smoke_reports_match_fixture_baseline.md) |
