---
okf_version: "0.2"
type: Module
title: snxwv
description: "Binary Columnar Waveform Streamer & GPU Decimation (`.snxwv`)."
resource: crates/oxide-sim/src/analysis/snxwv.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:36Z"
concept_id: crates/oxide-sim/src/analysis/snxwv
language: rust
---

# snxwv

Binary Columnar Waveform Streamer & GPU Decimation (`.snxwv`).

## Docstring

Binary Columnar Waveform Streamer & GPU Decimation (`.snxwv`).

Conforms to Master Technical Directive §3.8:
- Binary columnar container format with chunked pages for simulation traces exceeding $10^9$ points.
- Sub-pixel min/max decimation pipeline for ultra-fast viewport waveform rendering.

## Relationships

| Type | Target |
|------|--------|
| related | [SnxwvHeader](/crates/oxide-sim/src/analysis/snxwv/SnxwvHeader.md) |
| related | [default](/crates/oxide-sim/src/analysis/snxwv/default.md) |
| related | [default](/crates/oxide-sim/src/analysis/snxwv/default.md) |
| related | [DecimationBucket](/crates/oxide-sim/src/analysis/snxwv/DecimationBucket.md) |
| related | [WaveformDecimator](/crates/oxide-sim/src/analysis/snxwv/WaveformDecimator.md) |
| related | [decimate](/crates/oxide-sim/src/analysis/snxwv/decimate.md) |
| related | [decimate](/crates/oxide-sim/src/analysis/snxwv/decimate.md) |
| related | [test_waveform_decimation_preserves_extrema](/crates/oxide-sim/src/analysis/snxwv/test_waveform_decimation_preserves_extrema.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
