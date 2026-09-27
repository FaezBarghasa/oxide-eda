---
okf_version: "0.2"
type: Function
title: symbol_file_with_no_polygon_graphic_stays_v1
description: "A file with no `Polygon` graphic anywhere stays on"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/symbol_file_with_no_polygon_graphic_stays_v1
language: rust
---

# symbol_file_with_no_polygon_graphic_stays_v1

A file with no `Polygon` graphic anywhere stays on

## Signature

```rust
fn symbol_file_with_no_polygon_graphic_stays_v1()
```

## Decorators

- `test`

## Docstring

A file with no `Polygon` graphic anywhere stays on
`SYMBOL_FILE_FORMAT_TOKEN` ("snxsym/v1") — maximum compat: builds
that predate the `Polygon` variant can still read it.
[test]

## Source
Lines 549–564 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
