---
okf_version: "0.2"
type: Function
title: upconvert_to_carrier
description: Generates time-domain RF modulated carrier signal from baseband I/Q symbols.
resource: crates/oxide-rf/src/modulation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:41Z"
concept_id: crates/oxide-rf/src/modulation/upconvert_to_carrier
language: rust
---

# upconvert_to_carrier

Generates time-domain RF modulated carrier signal from baseband I/Q symbols.

## Signature

```rust
impl Modulator { pub fn upconvert_to_carrier(
        symbols: &[IqSymbol],
        samples_per_symbol: usize,
        carrier_freq_hz: f64,
        sample_rate_hz: f64,
    ) -> (Vec<f64>, Vec<f64>) }
```

## Visibility

- `pub`

## Docstring

Generates time-domain RF modulated carrier signal from baseband I/Q symbols.

## Source
Lines 117–142 in `crates/oxide-rf/src/modulation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modulation](/crates/oxide-rf/src/modulation.md) |
