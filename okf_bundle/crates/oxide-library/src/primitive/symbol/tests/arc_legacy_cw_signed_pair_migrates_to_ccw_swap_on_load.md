---
okf_version: "0.2"
type: Function
title: arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load
description: "A legacy CW-signed pair (a raw drag delta, never reconciled into"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load
language: rust
---

# arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load

A legacy CW-signed pair (a raw drag delta, never reconciled into

## Signature

```rust
fn arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load()
```

## Decorators

- `test`

## Docstring

A legacy CW-signed pair (a raw drag delta, never reconciled into
`[0, 360)`) migrates on load: swapped and reduced, reproducing the
short arc a pre-normalization build's signed CPU draw actually
showed the user.
[test]

## Source
Lines 589–602 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
| calls | [arc_symbol](/crates/oxide-library/src/primitive/symbol/tests/arc_symbol.md) |
