---
okf_version: "0.2"
type: Function
title: adjust_amplitude
description: "Adjust amplitude with hotkeys (e.g. '1' = +100µm, '2' = -100µm)."
resource: crates/oxide-router/src/interactive/session.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:42:20Z"
concept_id: crates/oxide-router/src/interactive/session/adjust_amplitude
language: rust
---

# adjust_amplitude

Adjust amplitude with hotkeys (e.g. '1' = +100µm, '2' = -100µm).

## Signature

```rust
impl InteractiveTuningHudState { pub fn adjust_amplitude(&mut self, delta_microns: Microns) }
```

## Visibility

- `pub`

## Docstring

Adjust amplitude with hotkeys (e.g. '1' = +100µm, '2' = -100µm).

## Source
Lines 72–74 in `crates/oxide-router/src/interactive/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-router/src/interactive/session.md) |
