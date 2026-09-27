---
okf_version: "0.2"
type: Class
title: InteractiveTuningHudState
description: Live interactive HUD state for high-speed length and phase tuning.
resource: crates/oxide-router/src/interactive/session.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:42:20Z"
concept_id: crates/oxide-router/src/interactive/session/InteractiveTuningHudState
language: rust
---

# InteractiveTuningHudState

Live interactive HUD state for high-speed length and phase tuning.

## Signature

```rust
pub struct InteractiveTuningHudState
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Live interactive HUD state for high-speed length and phase tuning.
[derive(Debug, Clone, PartialEq)]

## Methods

- `target_length_microns`
- `current_length_microns`
- `package_delay_microns`
- `tolerance_microns`
- `amplitude_microns`
- `pitch_microns`
- `corner_style`

## Source
Lines 33–41 in `crates/oxide-router/src/interactive/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-router/src/interactive/session.md) |
