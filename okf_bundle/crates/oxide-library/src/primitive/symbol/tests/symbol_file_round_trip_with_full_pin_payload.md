---
okf_version: "0.2"
type: Function
title: symbol_file_round_trip_with_full_pin_payload
description: All-fields round-trip — every SymbolPin field gets a non-default
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/symbol_file_round_trip_with_full_pin_payload
language: rust
---

# symbol_file_round_trip_with_full_pin_payload

All-fields round-trip — every SymbolPin field gets a non-default

## Signature

```rust
fn symbol_file_round_trip_with_full_pin_payload()
```

## Decorators

- `test`

## Docstring

All-fields round-trip — every SymbolPin field gets a non-default
value so the TSV cell encoders / decoders are exercised end-to-end.
[test]

## Source
Lines 130–159 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
