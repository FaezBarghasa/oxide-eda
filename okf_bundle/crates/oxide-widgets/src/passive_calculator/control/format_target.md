---
okf_version: "0.2"
type: Function
title: format_target
description: "The \"Target\" metric, or [`TARGET_UNAVAILABLE`] when the target input"
resource: crates/oxide-widgets/src/passive_calculator/control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/control/format_target
language: rust
---

# format_target

The "Target" metric, or [`TARGET_UNAVAILABLE`] when the target input

## Signature

```rust
pub fn format_target(target: Option<f64>, kind: ComponentKind) -> String
```

## Visibility

- `pub`

## Docstring

The "Target" metric, or [`TARGET_UNAVAILABLE`] when the target input
is not a number.

## Source
Lines 473–478 in `crates/oxide-widgets/src/passive_calculator/control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [control](/crates/oxide-widgets/src/passive_calculator/control.md) |
| calls | [format_value](/crates/oxide-widgets/src/passive_calculator/network/format_value.md) |
