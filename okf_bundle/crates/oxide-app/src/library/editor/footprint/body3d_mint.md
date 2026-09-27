---
okf_version: "0.2"
type: Module
title: body3d_mint
description: Minimal 3D-body mint helpers. No wgpu — the CPU preview3d pane reads
resource: crates/oxide-app/src/library/editor/footprint/body3d_mint.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d_mint
language: rust
---

# body3d_mint

Minimal 3D-body mint helpers. No wgpu — the CPU preview3d pane reads

## Docstring

Minimal 3D-body mint helpers. No wgpu — the CPU preview3d pane reads
`Footprint::body_3d` directly, so populating it is immediately visible.
Real interactive 3D manipulation stays deferred (v2.x,
docs/internal/docs/PCB_3D_RENDER_PLAN.md).

## Relationships

| Type | Target |
|------|--------|
| related | [mint_box_from_courtyard](/crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_box_from_courtyard.md) |
| related | [mint_extruded_from_fab](/crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_extruded_from_fab.md) |
