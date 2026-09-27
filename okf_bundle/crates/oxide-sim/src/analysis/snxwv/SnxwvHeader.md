---
okf_version: "0.2"
type: Class
title: SnxwvHeader
description: "Header metadata for `.snxwv` waveform file."
resource: crates/oxide-sim/src/analysis/snxwv.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:36Z"
concept_id: crates/oxide-sim/src/analysis/snxwv/SnxwvHeader
language: rust
---

# SnxwvHeader

Header metadata for `.snxwv` waveform file.

## Signature

```rust
pub struct SnxwvHeader
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Header metadata for `.snxwv` waveform file.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `magic`
- `title`
- `sample_count`
- `channel_count`
- `time_min`
- `time_max`

## Source
Lines 11–18 in `crates/oxide-sim/src/analysis/snxwv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snxwv](/crates/oxide-sim/src/analysis/snxwv.md) |
