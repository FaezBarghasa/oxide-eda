---
okf_version: "0.2"
type: Function
title: fill_none_and_some_map_to_fill_type
description: "Fill is lossy by construction: `None -> FillType::None`, `Some(_) ->"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/fill_none_and_some_map_to_fill_type
language: rust
---

# fill_none_and_some_map_to_fill_type

Fill is lossy by construction: `None -> FillType::None`, `Some(_) ->

## Signature

```rust
fn fill_none_and_some_map_to_fill_type()
```

## Decorators

- `test`

## Docstring

Fill is lossy by construction: `None -> FillType::None`, `Some(_) ->
FillType::Background` — the RGBA colour itself is dropped.
[test]

## Source
Lines 237–271 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
