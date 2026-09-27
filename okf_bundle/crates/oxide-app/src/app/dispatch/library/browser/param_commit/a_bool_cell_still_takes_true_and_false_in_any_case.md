---
okf_version: "0.2"
type: Function
title: a_bool_cell_still_takes_true_and_false_in_any_case
description: "The accepted set is exactly what it was before #612 — the fix"
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_bool_cell_still_takes_true_and_false_in_any_case
language: rust
---

# a_bool_cell_still_takes_true_and_false_in_any_case

The accepted set is exactly what it was before #612 — the fix

## Signature

```rust
fn a_bool_cell_still_takes_true_and_false_in_any_case()
```

## Decorators

- `test`

## Docstring

The accepted set is exactly what it was before #612 — the fix
closes the retyping path without narrowing or widening what a
boolean cell takes.
[test]

## Source
Lines 192–211 in `crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit.md) |
