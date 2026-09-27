---
okf_version: "0.2"
type: Function
title: is_active_low
description: Detects whether a pin name represents active-low logic.
resource: crates/oxide-library/src/symbol/generator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:02:04Z"
concept_id: crates/oxide-library/src/symbol/generator/is_active_low_1
language: rust
---

# is_active_low

Detects whether a pin name represents active-low logic.

## Signature

```rust
pub fn is_active_low(name: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Detects whether a pin name represents active-low logic.

## Source
Lines 44–53 in `crates/oxide-library/src/symbol/generator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [generator](/crates/oxide-library/src/symbol/generator.md) |
