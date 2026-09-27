---
okf_version: "0.2"
type: Function
title: placement_input_clears_after_commit
description: "v0.24 Track D — `state.placement_input` clears to `None` once the"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_clears_after_commit
language: rust
---

# placement_input_clears_after_commit

v0.24 Track D — `state.placement_input` clears to `None` once the

## Signature

```rust
fn placement_input_clears_after_commit()
```

## Decorators

- `test`

## Docstring

v0.24 Track D — `state.placement_input` clears to `None` once the
click that consumed it commits. The user has to type again before
the next gesture step to keep the chain explicit.
[test]

## Source
Lines 365–431 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
