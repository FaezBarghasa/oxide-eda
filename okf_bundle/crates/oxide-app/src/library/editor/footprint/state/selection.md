---
okf_version: "0.2"
type: Module
title: selection
description: "The one accessor that resolves \"the current pad selection\"."
resource: crates/oxide-app/src/library/editor/footprint/state/selection.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection
language: rust
---

# selection

The one accessor that resolves "the current pad selection".

## Docstring

The one accessor that resolves "the current pad selection".

`selected_pad` (the primary) and `selected_pads_extra` (the
ctrl-click additions) are two fields, and every op that acts on
the selection has to union them. Hand-rolling that union at each
call site is how the active-bar Rotate / Flip / Align-to-Grid
transforms ended up reading `selected_pad` alone and silently
acting on one pad out of N — a mixed-layer flip is a fab error, not
a cosmetic one. Read the selection through here.

## Relationships

| Type | Target |
|------|--------|
| related | [selected_pad_indices](/crates/oxide-app/src/library/editor/footprint/state/selection/selected_pad_indices.md) |
| related | [selected_pad_indices](/crates/oxide-app/src/library/editor/footprint/state/selection/selected_pad_indices.md) |
