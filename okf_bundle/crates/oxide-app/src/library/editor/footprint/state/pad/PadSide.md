---
okf_version: "0.2"
type: Class
title: PadSide
description: Pad copper side mirror — UI-side label-bearing enum. The sketch
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/PadSide
language: rust
---

# PadSide

Pad copper side mirror — UI-side label-bearing enum. The sketch

## Signature

```rust
pub enum PadSide
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

Pad copper side mirror — UI-side label-bearing enum. The sketch
crate has the same shape at `oxide_sketch::attr::PadSide`; this
type wraps it for the app's panel/dispatcher boundary so the panel
doesn't pull in the sketch crate's constraint-residual surface.

HI-24: variants MUST stay in lockstep with `oxide_sketch::attr::PadSide`.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]

## Source
Lines 402–407 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
