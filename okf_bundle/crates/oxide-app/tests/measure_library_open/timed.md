---
okf_version: "0.2"
type: Function
title: timed
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/timed
language: rust
---

# timed

## Signature

```rust
fn timed(f: impl FnOnce() -> T) -> (T, Duration)
```

## Type Parameters

- `T`

## Source
Lines 91–95 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
| called_by | [measure_scale](/crates/oxide-app/tests/measure_library_open/measure_scale.md) |
