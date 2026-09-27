---
okf_version: "0.2"
type: Function
title: compute_target_impedance
description: "Computes $Z_{\\text{target}} = \\frac{V_{\\text{dd}} \\cdot \\Delta V_{\\text{ripple}}}{I_{\\text{transient}}}$ in Ohms."
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/compute_target_impedance_1
language: rust
---

# compute_target_impedance

Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$ in Ohms.

## Signature

```rust
pub fn compute_target_impedance(&self) -> f64
```

## Visibility

- `pub`

## Docstring

Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$ in Ohms.

## Source
Lines 32–34 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
