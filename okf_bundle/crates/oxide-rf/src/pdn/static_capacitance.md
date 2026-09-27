---
okf_version: "0.2"
type: Function
title: static_capacitance
description: "Computes static plane capacitance $C_{\\text{plane}} = \\varepsilon_0 \\varepsilon_r \\frac{A}{d}$."
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/static_capacitance
language: rust
---

# static_capacitance

Computes static plane capacitance $C_{\text{plane}} = \varepsilon_0 \varepsilon_r \frac{A}{d}$.

## Signature

```rust
impl PowerPlaneCavity { pub fn static_capacitance(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Computes static plane capacitance $C_{\text{plane}} = \varepsilon_0 \varepsilon_r \frac{A}{d}$.

## Source
Lines 84–88 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
