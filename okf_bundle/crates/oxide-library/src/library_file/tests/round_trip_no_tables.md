---
okf_version: "0.2"
type: Function
title: round_trip_no_tables
description: The foundational round-trip — a library with no tables still
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/round_trip_no_tables
language: rust
---

# round_trip_no_tables

The foundational round-trip — a library with no tables still

## Signature

```rust
fn round_trip_no_tables()
```

## Decorators

- `test`

## Docstring

The foundational round-trip — a library with no tables still
preserves the manifest header byte-equal under `parse(write())`.
[test]

## Source
Lines 57–65 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| calls | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
