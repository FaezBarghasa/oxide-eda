---
okf_version: "0.2"
type: Function
title: specificity
description: Specificity priority rank. Higher numbers override lower numbers.
resource: crates/oxide-rules/src/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:31:50Z"
concept_id: crates/oxide-rules/src/scope/specificity_1
language: rust
---

# specificity

Specificity priority rank. Higher numbers override lower numbers.

## Signature

```rust
pub fn specificity(&self) -> u8
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Specificity priority rank. Higher numbers override lower numbers.
[inline]

## Source
Lines 97–104 in `crates/oxide-rules/src/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-rules/src/scope.md) |
