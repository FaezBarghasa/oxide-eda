---
okf_version: "0.2"
type: Function
title: is_available
description: Check if the ngspice binary is executable.
resource: crates/oxide-sim/src/simulator/ngspice.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:14:51Z"
concept_id: crates/oxide-sim/src/simulator/ngspice/is_available_1
language: rust
---

# is_available

Check if the ngspice binary is executable.

## Signature

```rust
pub fn is_available(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Check if the ngspice binary is executable.

## Source
Lines 31–38 in `crates/oxide-sim/src/simulator/ngspice.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ngspice](/crates/oxide-sim/src/simulator/ngspice.md) |
