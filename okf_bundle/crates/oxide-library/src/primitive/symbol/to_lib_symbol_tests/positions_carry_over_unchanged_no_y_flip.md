---
okf_version: "0.2"
type: Function
title: positions_carry_over_unchanged_no_y_flip
description: "Acceptance criterion: pin and graphic positions are byte-identical"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/positions_carry_over_unchanged_no_y_flip
language: rust
---

# positions_carry_over_unchanged_no_y_flip

Acceptance criterion: pin and graphic positions are byte-identical

## Signature

```rust
fn positions_carry_over_unchanged_no_y_flip()
```

## Decorators

- `test`

## Docstring

Acceptance criterion: pin and graphic positions are byte-identical
across the conversion — no y-flip happens here (that's
`SymbolTransform::apply`'s job, later, at place time).
[test]

## Source
Lines 87–106 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
| calls | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
