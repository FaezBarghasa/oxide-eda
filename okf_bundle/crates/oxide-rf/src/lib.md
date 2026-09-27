---
okf_version: "0.2"
type: Module
title: lib
description: "`oxide-rf` — Telecommunications, RF & Signal Integrity Simulation Engine for Oxide EDA."
resource: crates/oxide-rf/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:40:20Z"
concept_id: crates/oxide-rf/src/lib
language: rust
---

# lib

`oxide-rf` — Telecommunications, RF & Signal Integrity Simulation Engine for Oxide EDA.

## Docstring

`oxide-rf` — Telecommunications, RF & Signal Integrity Simulation Engine for Oxide EDA.

Provides:
- [`SParameters2Port`] & [`SParameterDataset`]: 2-port network analysis, return/insertion loss, VSWR, Touchstone export, and Smith chart projections.
- [`ModulationScheme`] & [`Modulator`]: Digital/Analog modulation (AM, FM, ASK, FSK, BPSK, QPSK, 16/64/256-QAM) and I/Q baseband symbol generation.
- [`EyeDiagramDataset`]: Eye diagram windowing and signal integrity metrics (eye height, eye width, jitter RMS/p2p, SNR).
- [`ConstellationDataset`]: I/Q constellation mapping and Error Vector Magnitude (EVM) calculation.
- [`ChannelModel`]: AWGN noise, path loss, and phase/frequency offset impairment models.
- [`BerCurve`]: Bit Error Rate waterfall curves vs. $E_b/N_0$.

## Relationships

| Type | Target |
|------|--------|
| related | [test_s_parameters_pi_attenuator](/crates/oxide-rf/src/lib/test_s_parameters_pi_attenuator.md) |
| related | [test_modulation_and_constellation](/crates/oxide-rf/src/lib/test_modulation_and_constellation.md) |
| related | [test_eye_diagram_measurements](/crates/oxide-rf/src/lib/test_eye_diagram_measurements.md) |
