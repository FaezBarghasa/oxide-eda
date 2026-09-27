---
okf_version: "0.2"
type: Function
title: arc_full_turn_migrates_to_circle_on_load
description: "An exact, nonzero 360° span (legacy full-circle authoring, or a"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/arc_full_turn_migrates_to_circle_on_load
language: rust
---

# arc_full_turn_migrates_to_circle_on_load

An exact, nonzero 360° span (legacy full-circle authoring, or a

## Signature

```rust
fn arc_full_turn_migrates_to_circle_on_load()
```

## Decorators

- `test`

## Docstring

An exact, nonzero 360° span (legacy full-circle authoring, or a
drag/rotation that landed on exactly one full turn) migrates to a
`Circle` on load instead of computing a zero CCW-wraparound sweep
and drawing nothing.
[test]

## Source
Lines 654–677 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
