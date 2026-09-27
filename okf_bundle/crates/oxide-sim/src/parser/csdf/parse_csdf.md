---
okf_version: "0.2"
type: Function
title: parse_csdf
description: "Parse a Cadence PSpice CSDF simulation output text into a [`WaveformDataset`]."
resource: crates/oxide-sim/src/parser/csdf.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:16:33Z"
concept_id: crates/oxide-sim/src/parser/csdf/parse_csdf
language: rust
---

# parse_csdf

Parse a Cadence PSpice CSDF simulation output text into a [`WaveformDataset`].

## Signature

```rust
pub fn parse_csdf(content: &str) -> Result<WaveformDataset, SimError>
```

## Visibility

- `pub`

## Docstring

Parse a Cadence PSpice CSDF simulation output text into a [`WaveformDataset`].

## Source
Lines 8–95 in `crates/oxide-sim/src/parser/csdf.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [csdf](/crates/oxide-sim/src/parser/csdf.md) |
| called_by | [parse_csdf_sample](/crates/oxide-sim/src/parser/csdf/parse_csdf_sample.md) |
| called_by | [run](/crates/oxide-sim/src/simulator/pspice_cli/run.md) |
