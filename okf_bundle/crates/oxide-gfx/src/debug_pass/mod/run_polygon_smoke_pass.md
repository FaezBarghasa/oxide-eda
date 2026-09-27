---
okf_version: "0.2"
type: Function
title: run_polygon_smoke_pass
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_polygon_smoke_pass
language: rust
---

# run_polygon_smoke_pass

## Signature

```rust
pub fn run_polygon_smoke_pass() -> Result<u32, String>
```

## Visibility

- `pub`

## Source
Lines 510–519 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| calls | [run_polygon_smoke_pass_with](/crates/oxide-gfx/src/debug_pass/mod/run_polygon_smoke_pass_with.md) |
| called_by | [polygon_smoke_pass_runs](/crates/oxide-gfx/src/debug_pass/tests/polygon_smoke_pass_runs.md) |
| called_by | [regression_golden_smoke_reports_match_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_smoke_reports_match_fixture_baseline.md) |
