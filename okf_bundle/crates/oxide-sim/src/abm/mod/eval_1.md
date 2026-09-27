---
okf_version: "0.2"
type: Function
title: eval
description: Evaluates the expression value given the current circuit node voltages.
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod/eval_1
language: rust
---

# eval

Evaluates the expression value given the current circuit node voltages.

## Signature

```rust
pub fn eval(&self, state: &[f64]) -> f64
```

## Visibility

- `pub`

## Docstring

Evaluates the expression value given the current circuit node voltages.

## Source
Lines 130–141 in `crates/oxide-sim/src/abm/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abm](/crates/oxide-sim/src/abm/mod.md) |
