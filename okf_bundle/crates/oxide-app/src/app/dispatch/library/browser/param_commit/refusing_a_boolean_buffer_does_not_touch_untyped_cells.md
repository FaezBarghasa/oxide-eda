---
okf_version: "0.2"
type: Function
title: refusing_a_boolean_buffer_does_not_touch_untyped_cells
description: "A cell that is not typed boolean is unaffected: `\"maybe\"` is a"
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusing_a_boolean_buffer_does_not_touch_untyped_cells
language: rust
---

# refusing_a_boolean_buffer_does_not_touch_untyped_cells

A cell that is not typed boolean is unaffected: `"maybe"` is a

## Signature

```rust
fn refusing_a_boolean_buffer_does_not_touch_untyped_cells()
```

## Decorators

- `test`

## Docstring

A cell that is not typed boolean is unaffected: `"maybe"` is a
perfectly good text value and must still commit.
[test]

## Source
Lines 216–222 in `crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit.md) |
| calls | [param_value_for_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit.md) |
