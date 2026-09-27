---
okf_version: "0.2"
type: Function
title: graphic_missing_part_number_defaults_to_zero
description: "A graphic left at the default (`0` = shared / drawn on every unit)"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/graphic_missing_part_number_defaults_to_zero
language: rust
---

# graphic_missing_part_number_defaults_to_zero

A graphic left at the default (`0` = shared / drawn on every unit)

## Signature

```rust
fn graphic_missing_part_number_defaults_to_zero()
```

## Decorators

- `test`

## Docstring

A graphic left at the default (`0` = shared / drawn on every unit)
reloads as `0` — proves the additive, back-compatible default so
pre-C1 files whose graphics carried no part scoping render as before.
[test]

## Source
Lines 491–506 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
