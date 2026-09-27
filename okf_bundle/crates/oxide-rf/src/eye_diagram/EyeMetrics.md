---
okf_version: "0.2"
type: Class
title: EyeMetrics
description: Measured Eye Diagram Quality Metrics.
resource: crates/oxide-rf/src/eye_diagram.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:47Z"
concept_id: crates/oxide-rf/src/eye_diagram/EyeMetrics
language: rust
---

# EyeMetrics

Measured Eye Diagram Quality Metrics.

## Signature

```rust
pub struct EyeMetrics
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Measured Eye Diagram Quality Metrics.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `eye_height`
- `eye_width_s`
- `eye_opening_ratio`
- `jitter_rms_s`
- `jitter_p2p_s`
- `snr_eye_db`

## Source
Lines 7–14 in `crates/oxide-rf/src/eye_diagram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eye_diagram](/crates/oxide-rf/src/eye_diagram.md) |
