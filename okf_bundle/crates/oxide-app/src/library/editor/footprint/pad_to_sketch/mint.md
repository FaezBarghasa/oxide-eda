---
okf_version: "0.2"
type: Module
title: mint
description: "Per-shape parametric geometry minting. Each `mint_*_pad_geometry`"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint
language: rust
---

# mint

Per-shape parametric geometry minting. Each `mint_*_pad_geometry`

## Docstring

Per-shape parametric geometry minting. Each `mint_*_pad_geometry`
function takes a fresh centre `Point` ID (pushed by the caller),
mints additional geometry (Lines / Arcs / Circle / extra Points)
and the matching shape parameters, and returns the bbox-corner IDs
that go into `EditorPad.corner_entity_ids`.

## Relationships

| Type | Target |
|------|--------|
| related | [mint_pad_corner_outline](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_pad_corner_outline.md) |
| related | [mint_round_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_pad_geometry.md) |
| related | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
| related | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| related | [mint_chamfered_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_chamfered_pad_geometry.md) |
