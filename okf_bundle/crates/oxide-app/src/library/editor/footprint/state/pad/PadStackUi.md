---
okf_version: "0.2"
type: Class
title: PadStackUi
description: "v0.20 — UI-side mirror of `Pad`'s pad-stack override fields. All"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/PadStackUi
language: rust
---

# PadStackUi

v0.20 — UI-side mirror of `Pad`'s pad-stack override fields. All

## Signature

```rust
pub struct PadStackUi
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

v0.20 — UI-side mirror of `Pad`'s pad-stack override fields. All
values in mm (already evaluated). `None` on a margin override
means "use the rule-driven / global value"; `true` on a tented
flag means "skip the mask opening on that side".
[derive(Debug, Clone, PartialEq)]

## Methods

- `paste_margin_top`
- `paste_margin_bottom`
- `paste_enabled_top`
- `paste_enabled_bottom`
- `mask_margin_top`
- `mask_margin_bottom`
- `mask_tented_top`
- `mask_tented_bottom`
- `thermal_relief`
- `corner_radius_pct`

## Source
Lines 75–86 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
