---
okf_version: "0.2"
type: Class
title: Simulator
description: The abstraction port for external circuit simulators (ADR-0001 §A3.2).
resource: crates/oxide-sim/src/simulator/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:13:06Z"
concept_id: crates/oxide-sim/src/simulator/mod/Simulator
language: rust
---

# Simulator

The abstraction port for external circuit simulators (ADR-0001 §A3.2).

## Signature

```rust
pub trait Simulator
```

## Visibility

- `pub`

## Docstring

The abstraction port for external circuit simulators (ADR-0001 §A3.2).

## Source
Lines 49–65 in `crates/oxide-sim/src/simulator/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/oxide-sim/src/simulator/mod.md) |
