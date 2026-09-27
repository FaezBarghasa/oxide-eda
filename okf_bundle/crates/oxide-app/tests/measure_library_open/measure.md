---
okf_version: "0.2"
type: Function
title: measure
description: "Run `f` `RUNS + 1` times, discard the first (warm-up), keep the rest."
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/measure
language: rust
---

# measure

Run `f` `RUNS + 1` times, discard the first (warm-up), keep the rest.

## Signature

```rust
fn measure(scale: &str, op: &str, rows: &mut Vec<Row>, mut f: impl FnMut() -> Duration)
```

## Docstring

Run `f` `RUNS + 1` times, discard the first (warm-up), keep the rest.
`f` returns only the duration of the region under test so per-iteration
setup (fresh `LibraryState`, fresh adapter) stays out of the number.

## Source
Lines 71–89 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
| called_by | [measure_scale](/crates/oxide-app/tests/measure_library_open/measure_scale.md) |
