---
okf_version: "0.2"
type: Function
title: synthesize_thermal_paste_panes
description: "Synthesizes paste aperture window panes for thermal pad (achieving 60-70% coverage)."
resource: crates/oxide-bake/src/ipc7351/calculator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:10:00Z"
concept_id: crates/oxide-bake/src/ipc7351/calculator/synthesize_thermal_paste_panes
language: rust
---

# synthesize_thermal_paste_panes

Synthesizes paste aperture window panes for thermal pad (achieving 60-70% coverage).

## Signature

```rust
pub fn synthesize_thermal_paste_panes(
    thermal_w: f64,
    thermal_h: f64,
    target_coverage: f64, // e.g. 0.65 (65%)
) -> Vec<FpPasteAperture>
```

## Visibility

- `pub`

## Docstring

Synthesizes paste aperture window panes for thermal pad (achieving 60-70% coverage).

## Source
Lines 164–227 in `crates/oxide-bake/src/ipc7351/calculator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [calculator](/crates/oxide-bake/src/ipc7351/calculator.md) |
| called_by | [generate_footprint](/crates/oxide-bake/src/ipc7351/mod/generate_footprint.md) |
