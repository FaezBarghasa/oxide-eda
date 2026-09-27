---
okf_version: "0.2"
type: Function
title: length_delta
description: "Deviation from target length in micrometers (+ is over, - is under)."
resource: crates/oxide-router/src/interactive/session.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:42:20Z"
concept_id: crates/oxide-router/src/interactive/session/length_delta
language: rust
---

# length_delta

Deviation from target length in micrometers (+ is over, - is under).

## Signature

```rust
impl InteractiveTuningHudState { pub fn length_delta(&self) -> Microns }
```

## Visibility

- `pub`

## Docstring

Deviation from target length in micrometers (+ is over, - is under).

## Source
Lines 62–64 in `crates/oxide-router/src/interactive/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-router/src/interactive/session.md) |
