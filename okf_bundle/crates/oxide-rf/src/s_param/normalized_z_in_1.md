---
okf_version: "0.2"
type: Function
title: normalized_z_in
description: "Converts $S_{11}$ reflection coefficient $\\Gamma$ into normalized input impedance:"
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/normalized_z_in_1
language: rust
---

# normalized_z_in

Converts $S_{11}$ reflection coefficient $\Gamma$ into normalized input impedance:

## Signature

```rust
pub fn normalized_z_in(&self) -> Option<Complex64>
```

## Visibility

- `pub`

## Docstring

Converts $S_{11}$ reflection coefficient $\Gamma$ into normalized input impedance:
$z_{in} = \frac{1 + \Gamma}{1 - \Gamma}$

## Source
Lines 115–119 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
