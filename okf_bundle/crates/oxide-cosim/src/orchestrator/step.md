---
okf_version: "0.2"
type: Function
title: step
description: Advances the co-simulation timeline by one hybrid step $\Delta t$.
resource: crates/oxide-cosim/src/orchestrator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:33:10Z"
concept_id: crates/oxide-cosim/src/orchestrator/step
language: rust
---

# step

Advances the co-simulation timeline by one hybrid step $\Delta t$.

## Signature

```rust
impl CoSimOrchestrator { pub fn step(&mut self) }
```

## Visibility

- `pub`

## Docstring

Advances the co-simulation timeline by one hybrid step $\Delta t$.

## Source
Lines 38–54 in `crates/oxide-cosim/src/orchestrator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [orchestrator](/crates/oxide-cosim/src/orchestrator.md) |
