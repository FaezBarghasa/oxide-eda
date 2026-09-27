---
okf_version: "0.2"
type: Function
title: insertion_loss_db
description: "Insertion Loss (dB) from Port 1 to Port 2: $IL = -20 \\log_{10} |S_{21}|$"
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/insertion_loss_db_1
language: rust
---

# insertion_loss_db

Insertion Loss (dB) from Port 1 to Port 2: $IL = -20 \log_{10} |S_{21}|$

## Signature

```rust
pub fn insertion_loss_db(&self) -> f64
```

## Visibility

- `pub`

## Docstring

Insertion Loss (dB) from Port 1 to Port 2: $IL = -20 \log_{10} |S_{21}|$

## Source
Lines 99–101 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
