---
okf_version: "0.2"
type: Function
title: advance_fallback_stage
description: Advances to the next fallback recovery stage upon non-linear divergence.
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade/advance_fallback_stage
language: rust
---

# advance_fallback_stage

Advances to the next fallback recovery stage upon non-linear divergence.

## Signature

```rust
impl ConvergenceCascade { pub fn advance_fallback_stage(&mut self) -> Option<ConvergenceStage> }
```

## Visibility

- `pub`

## Docstring

Advances to the next fallback recovery stage upon non-linear divergence.

## Source
Lines 59–82 in `crates/oxide-sim/src/engine/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-sim/src/engine/cascade.md) |
