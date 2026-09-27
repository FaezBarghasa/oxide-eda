---
okf_version: "0.2"
type: Function
title: measure_scale
description: ── Per-library measurements ─────────────────────────────────────────────
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/measure_scale
language: rust
---

# measure_scale

── Per-library measurements ─────────────────────────────────────────────

## Signature

```rust
fn measure_scale(scale: &Scale, snxlib: &Path, rows: &mut Vec<Row>)
```

## Docstring

── Per-library measurements ─────────────────────────────────────────────

## Source
Lines 99–179 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| calls | [measure](/crates/oxide-app/tests/measure_library_open/measure.md) |
| calls | [timed](/crates/oxide-app/tests/measure_library_open/timed.md) |
| calls | [blank_open_library](/crates/oxide-app/tests/measure_library_open/blank_open_library.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
