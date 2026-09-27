---
okf_version: "0.2"
type: Function
title: solver_max_iters_field_is_honoured
description: "Drives `max_iters = 1` and verifies the solver caps at exactly that"
resource: crates/oxide-sketch/tests/solver_api.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/solver_api/solver_max_iters_field_is_honoured
language: rust
---

# solver_max_iters_field_is_honoured

Drives `max_iters = 1` and verifies the solver caps at exactly that

## Signature

```rust
fn solver_max_iters_field_is_honoured()
```

## Decorators

- `test`

## Docstring

Drives `max_iters = 1` and verifies the solver caps at exactly that
many iterations — guards against the regression where the field was
silently ignored in favour of a module-level `MAX_ITERS` constant.
[test]

## Source
Lines 179–211 in `crates/oxide-sketch/tests/solver_api.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_api](/crates/oxide-sketch/tests/solver_api.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/solver_api/empty_params.md) |
