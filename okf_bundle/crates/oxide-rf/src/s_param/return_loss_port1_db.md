---
okf_version: "0.2"
type: Function
title: return_loss_port1_db
description: "Return Loss (dB) at Port 1: $RL_1 = -20 \\log_{10} |S_{11}|$"
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/return_loss_port1_db
language: rust
---

# return_loss_port1_db

Return Loss (dB) at Port 1: $RL_1 = -20 \log_{10} |S_{11}|$

## Signature

```rust
impl SParameters2Port { pub fn return_loss_port1_db(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Return Loss (dB) at Port 1: $RL_1 = -20 \log_{10} |S_{11}|$

## Source
Lines 94–96 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
