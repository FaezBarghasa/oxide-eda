---
okf_version: "0.2"
type: Function
title: erfc
description: Complementary error function approximation.
resource: crates/oxide-rf/src/ber.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:27:13Z"
concept_id: crates/oxide-rf/src/ber/erfc
language: rust
---

# erfc

Complementary error function approximation.

## Signature

```rust
fn erfc(x: f64) -> f64
```

## Docstring

Complementary error function approximation.

## Source
Lines 47–61 in `crates/oxide-rf/src/ber.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ber](/crates/oxide-rf/src/ber.md) |
| called_by | [theoretical_bpsk_awgn](/crates/oxide-rf/src/ber/theoretical_bpsk_awgn.md) |
