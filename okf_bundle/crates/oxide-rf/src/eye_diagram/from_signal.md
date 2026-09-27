---
okf_version: "0.2"
type: Function
title: from_signal
description: Generates 2-UI eye diagram windows from time-domain signal and calculates SI metrics.
resource: crates/oxide-rf/src/eye_diagram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:47Z"
concept_id: crates/oxide-rf/src/eye_diagram/from_signal
language: rust
---

# from_signal

Generates 2-UI eye diagram windows from time-domain signal and calculates SI metrics.

## Signature

```rust
impl EyeDiagramDataset { pub fn from_signal(time: &[f64], voltage: &[f64], symbol_rate_baud: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Generates 2-UI eye diagram windows from time-domain signal and calculates SI metrics.

## Source
Lines 34–130 in `crates/oxide-rf/src/eye_diagram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eye_diagram](/crates/oxide-rf/src/eye_diagram.md) |
