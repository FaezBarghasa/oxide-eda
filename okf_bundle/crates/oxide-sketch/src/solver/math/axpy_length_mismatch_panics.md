---
okf_version: "0.2"
type: Function
title: axpy_length_mismatch_panics
description: "[test]"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/axpy_length_mismatch_panics
language: rust
---

# axpy_length_mismatch_panics

[test]

## Signature

```rust
fn axpy_length_mismatch_panics()
```

## Decorators

- `test`
- `should_panic(expected = "axpy: length mismatch")`

## Docstring

[test]
[should_panic(expected = "axpy: length mismatch")]

## Source
Lines 344–348 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| calls | [axpy](/crates/oxide-sketch/src/solver/math/axpy.md) |
