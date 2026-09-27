---
okf_version: "0.2"
type: Function
title: solver_tolerance_field_is_honoured
description: "Drives `tolerance = 10.0` (an absurdly loose bound). The solver"
resource: crates/oxide-sketch/tests/solver_api.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/solver_api/solver_tolerance_field_is_honoured
language: rust
---

# solver_tolerance_field_is_honoured

Drives `tolerance = 10.0` (an absurdly loose bound). The solver

## Signature

```rust
fn solver_tolerance_field_is_honoured()
```

## Decorators

- `test`

## Docstring

Drives `tolerance = 10.0` (an absurdly loose bound). The solver
should declare convergence on the first iteration because `|r|² < 100`
is trivially true for the canonical anchored-distance case.
[test]

## Source
Lines 217–254 in `crates/oxide-sketch/tests/solver_api.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_api](/crates/oxide-sketch/tests/solver_api.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/solver_api/empty_params.md) |
