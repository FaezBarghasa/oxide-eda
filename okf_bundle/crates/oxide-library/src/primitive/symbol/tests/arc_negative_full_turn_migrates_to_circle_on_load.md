---
okf_version: "0.2"
type: Function
title: arc_negative_full_turn_migrates_to_circle_on_load
description: A negative exact-360° span also converts (the full-turn check runs
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/arc_negative_full_turn_migrates_to_circle_on_load
language: rust
---

# arc_negative_full_turn_migrates_to_circle_on_load

A negative exact-360° span also converts (the full-turn check runs

## Signature

```rust
fn arc_negative_full_turn_migrates_to_circle_on_load()
```

## Decorators

- `test`

## Docstring

A negative exact-360° span also converts (the full-turn check runs
before the CW-signed swap, which would otherwise collapse this to
a degenerate `start == end` point-arc instead of a visible circle).
[test]

## Source
Lines 683–691 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
| calls | [arc_symbol](/crates/oxide-library/src/primitive/symbol/tests/arc_symbol.md) |
