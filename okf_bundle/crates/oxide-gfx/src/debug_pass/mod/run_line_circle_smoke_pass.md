---
okf_version: "0.2"
type: Function
title: run_line_circle_smoke_pass
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_line_circle_smoke_pass
language: rust
---

# run_line_circle_smoke_pass

## Signature

```rust
pub fn run_line_circle_smoke_pass(scale_px_per_mm: f32) -> Result<SmokePassReport, String>
```

## Visibility

- `pub`

## Source
Lines 70–182 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| called_by | [line_circle_smoke_pass_runs_for_multiple_scales](/crates/oxide-gfx/src/debug_pass/tests/line_circle_smoke_pass_runs_for_multiple_scales.md) |
| called_by | [regression_golden_smoke_reports_match_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_smoke_reports_match_fixture_baseline.md) |
