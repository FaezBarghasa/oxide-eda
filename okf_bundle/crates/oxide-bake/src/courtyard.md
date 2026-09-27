---
okf_version: "0.2"
type: Module
title: courtyard
description: Courtyard bake — turns CourtyardAttr-tagged closed-profile sketches
resource: crates/oxide-bake/src/courtyard.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/courtyard
language: rust
---

# courtyard

Courtyard bake — turns CourtyardAttr-tagged closed-profile sketches

## Docstring

Courtyard bake — turns CourtyardAttr-tagged closed-profile sketches
into the footprint's `courtyard: Polygon` field.

Phase B / Stage 3 of the v0.14 sketch-mode plan. The courtyard is
a single polygon (IPC-7351 calls it the courtyard outline) that
tells PCB DRC how much room the part claims. Per the v3 schema,
`Footprint::courtyard: Polygon` is a single polygon — only the
first closed profile tagged with [`CourtyardAttr`] becomes the
courtyard; subsequent tagged profiles emit a warning.

v0.14 scope:
- One CourtyardAttr per footprint becomes the courtyard polygon.
- Additional CourtyardAttr-tagged entities warn + skip.
- Open / branching / arc-containing profiles surface a warning
from `oxide_bake::trace_closed_profile` and skip.
- Construction entities are excluded from the trace by the walker.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_courtyard](/crates/oxide-bake/src/courtyard/bake_courtyard.md) |
| related | [solve](/crates/oxide-bake/src/courtyard/solve.md) |
| related | [rectangle_with_courtyard](/crates/oxide-bake/src/courtyard/rectangle_with_courtyard.md) |
| related | [bake_courtyard_rectangle](/crates/oxide-bake/src/courtyard/bake_courtyard_rectangle.md) |
| related | [bake_courtyard_second_attr_warns](/crates/oxide-bake/src/courtyard/bake_courtyard_second_attr_warns.md) |
| related | [bake_courtyard_open_chain_warns](/crates/oxide-bake/src/courtyard/bake_courtyard_open_chain_warns.md) |
| related | [bake_courtyard_construction_seed_skipped](/crates/oxide-bake/src/courtyard/bake_courtyard_construction_seed_skipped.md) |
