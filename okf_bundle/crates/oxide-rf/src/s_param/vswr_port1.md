---
okf_version: "0.2"
type: Function
title: vswr_port1
description: "Voltage Standing Wave Ratio (VSWR) at Port 1: $VSWR = \\frac{1 + |S_{11}|}{1 - |S_{11}|}$"
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/vswr_port1
language: rust
---

# vswr_port1

Voltage Standing Wave Ratio (VSWR) at Port 1: $VSWR = \frac{1 + |S_{11}|}{1 - |S_{11}|}$

## Signature

```rust
impl SParameters2Port { pub fn vswr_port1(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Voltage Standing Wave Ratio (VSWR) at Port 1: $VSWR = \frac{1 + |S_{11}|}{1 - |S_{11}|}$

## Source
Lines 104–111 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
