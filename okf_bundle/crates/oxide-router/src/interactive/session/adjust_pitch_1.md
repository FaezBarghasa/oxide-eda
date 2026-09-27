---
okf_version: "0.2"
type: Function
title: adjust_pitch
description: "Adjust pitch/wavelength with hotkeys (e.g. '3' = +100µm, '4' = -100µm)."
resource: crates/oxide-router/src/interactive/session.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:42:20Z"
concept_id: crates/oxide-router/src/interactive/session/adjust_pitch_1
language: rust
---

# adjust_pitch

Adjust pitch/wavelength with hotkeys (e.g. '3' = +100µm, '4' = -100µm).

## Signature

```rust
pub fn adjust_pitch(&mut self, delta_microns: Microns)
```

## Visibility

- `pub`

## Docstring

Adjust pitch/wavelength with hotkeys (e.g. '3' = +100µm, '4' = -100µm).

## Source
Lines 77–79 in `crates/oxide-router/src/interactive/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-router/src/interactive/session.md) |
