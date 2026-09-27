---
okf_version: "0.2"
type: Function
title: format_optional_difference
description: "A delta against the target, or [`TARGET_UNAVAILABLE`] when there is"
resource: crates/oxide-widgets/src/passive_calculator/control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/control/format_optional_difference
language: rust
---

# format_optional_difference

A delta against the target, or [`TARGET_UNAVAILABLE`] when there is

## Signature

```rust
pub fn format_optional_difference(value: f64, target: Option<f64>) -> String
```

## Visibility

- `pub`

## Docstring

A delta against the target, or [`TARGET_UNAVAILABLE`] when there is
no readable target to measure against.

## Source
Lines 482–487 in `crates/oxide-widgets/src/passive_calculator/control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [control](/crates/oxide-widgets/src/passive_calculator/control.md) |
| calls | [format_difference](/crates/oxide-widgets/src/passive_calculator/control/format_difference.md) |
