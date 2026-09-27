---
okf_version: "0.2"
type: Function
title: from_model
description: "Wrap a single `SimModel` into a one-element file envelope."
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/from_model_1
language: rust
---

# from_model

Wrap a single `SimModel` into a one-element file envelope.

## Signature

```rust
pub fn from_model(model: SimModel) -> Self
```

## Visibility

- `pub`

## Docstring

Wrap a single `SimModel` into a one-element file envelope.

## Source
Lines 143–153 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
| calls | [default_sim_format](/crates/oxide-library/src/primitive/sim/default_sim_format.md) |
