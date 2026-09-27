---
okf_version: "0.2"
type: Function
title: theoretical_bpsk_awgn
description: "Theoretical BPSK/QPSK BER in AWGN: $P_b = \\frac{1}{2} \\text{erfc}\\left(\\sqrt{\\frac{E_b}{N_0}}\\right) = Q\\left(\\sqrt{\\frac{2E_b}{N_0}}\\right)$"
resource: crates/oxide-rf/src/ber.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:27:13Z"
concept_id: crates/oxide-rf/src/ber/theoretical_bpsk_awgn
language: rust
---

# theoretical_bpsk_awgn

Theoretical BPSK/QPSK BER in AWGN: $P_b = \frac{1}{2} \text{erfc}\left(\sqrt{\frac{E_b}{N_0}}\right) = Q\left(\sqrt{\frac{2E_b}{N_0}}\right)$

## Signature

```rust
impl BerCurve { pub fn theoretical_bpsk_awgn(eb_n0_db_range: &[f64]) -> Self }
```

## Visibility

- `pub`

## Docstring

Theoretical BPSK/QPSK BER in AWGN: $P_b = \frac{1}{2} \text{erfc}\left(\sqrt{\frac{E_b}{N_0}}\right) = Q\left(\sqrt{\frac{2E_b}{N_0}}\right)$

## Source
Lines 30–43 in `crates/oxide-rf/src/ber.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ber](/crates/oxide-rf/src/ber.md) |
| calls | [erfc](/crates/oxide-rf/src/ber/erfc.md) |
