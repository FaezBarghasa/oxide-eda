---
okf_version: "0.2"
type: Function
title: parse_spice_raw
description: "Parses a SPICE raw data byte buffer into a [`WaveformDataset`]."
resource: crates/oxide-sim/src/parser/raw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:00Z"
concept_id: crates/oxide-sim/src/parser/raw/parse_spice_raw
language: rust
---

# parse_spice_raw

Parses a SPICE raw data byte buffer into a [`WaveformDataset`].

## Signature

```rust
pub fn parse_spice_raw(bytes: &[u8]) -> Result<WaveformDataset, SimError>
```

## Visibility

- `pub`

## Docstring

Parses a SPICE raw data byte buffer into a [`WaveformDataset`].

## Source
Lines 8–171 in `crates/oxide-sim/src/parser/raw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [raw](/crates/oxide-sim/src/parser/raw.md) |
| called_by | [parse_ascii_spice_raw](/crates/oxide-sim/src/parser/raw/parse_ascii_spice_raw.md) |
| called_by | [run](/crates/oxide-sim/src/simulator/ngspice/run.md) |
