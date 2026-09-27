---
okf_version: "0.2"
type: Module
title: measure_library_open
description: "Timing probe for the library-open path — issue #99 part 2."
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open
language: rust
---

# measure_library_open

Timing probe for the library-open path — issue #99 part 2.

## Docstring

Timing probe for the library-open path — issue #99 part 2.

Not a pass/fail test: it prints numbers and asserts nothing about them,
so it is `#[ignore]`d and never runs in CI. It is committed so the
before/after of a change to the open path can be re-measured on demand
and a future regression has a ready-made probe. Run it explicitly:

```text
cargo test -p oxide-app --release --test measure_library_open -- --ignored --nocapture
```

`--release` is not optional. The debug profile is >10× slower and the
generation step alone will not finish in a reasonable time.

It generates synthetic `.snxlib` libraries at four scales in a tempdir
(via the real `LocalGitAdapter` / `SymbolFile` / `FootprintFile` /
`SimFile` writers, so nothing about the on-disk format is guessed) and
wall-clocks the project-open path against them. No production code is
touched; every call goes through an already-public API.

Behavioural coverage of the same path — that `open_library` really does
prime all five caches — lives in `tests/library_open_cache.rs`.

## Relationships

| Type | Target |
|------|--------|
| related | [Stat](/crates/oxide-app/tests/measure_library_open/Stat.md) |
| related | [from](/crates/oxide-app/tests/measure_library_open/from.md) |
| related | [from](/crates/oxide-app/tests/measure_library_open/from.md) |
| related | [Row](/crates/oxide-app/tests/measure_library_open/Row.md) |
| related | [measure](/crates/oxide-app/tests/measure_library_open/measure.md) |
| related | [timed](/crates/oxide-app/tests/measure_library_open/timed.md) |
| related | [measure_scale](/crates/oxide-app/tests/measure_library_open/measure_scale.md) |
| related | [blank_open_library](/crates/oxide-app/tests/measure_library_open/blank_open_library.md) |
| related | [write_project](/crates/oxide-app/tests/measure_library_open/write_project.md) |
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
