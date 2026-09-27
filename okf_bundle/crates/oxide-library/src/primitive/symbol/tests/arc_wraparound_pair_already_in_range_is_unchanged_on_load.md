---
okf_version: "0.2"
type: Function
title: arc_wraparound_pair_already_in_range_is_unchanged_on_load
description: "A wrapped pair with both endpoints already in `[0, 360)` — the"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/arc_wraparound_pair_already_in_range_is_unchanged_on_load
language: rust
---

# arc_wraparound_pair_already_in_range_is_unchanged_on_load

A wrapped pair with both endpoints already in `[0, 360)` — the

## Signature

```rust
fn arc_wraparound_pair_already_in_range_is_unchanged_on_load()
```

## Decorators

- `test`

## Docstring

A wrapped pair with both endpoints already in `[0, 360)` — the
form `rotation.rs`'s Arc rotate transform has always produced for
a 0°-crossing arc, meaning wraparound from day one — loads
unchanged.
[test]

## Source
Lines 609–622 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
| calls | [arc_symbol](/crates/oxide-library/src/primitive/symbol/tests/arc_symbol.md) |
