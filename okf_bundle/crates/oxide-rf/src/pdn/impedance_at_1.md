---
okf_version: "0.2"
type: Function
title: impedance_at
description: Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/impedance_at_1
language: rust
---

# impedance_at

Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.

## Signature

```rust
pub fn impedance_at(&self, freq_hz: f64) -> f64
```

## Visibility

- `pub`

## Docstring

Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.

## Source
Lines 59–69 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
