---
okf_version: "0.2"
type: Module
title: remint_in_place
description: The id-preserving form of the pad-sidecar re-mint.
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place
language: rust
---

# remint_in_place

The id-preserving form of the pad-sidecar re-mint.

## Docstring

The id-preserving form of the pad-sidecar re-mint.

[`super::remint_pad_geometry`] drops the pad's sketch geometry and
mints it afresh, which is right for a discrete command (rotate,
flip, a Properties-panel field) and wrong for a live drag: the
pointer holds the id of the entity the user grabbed for the whole
gesture. This module mints the same geometry through the same owner
and writes it ONTO the entities already there.

## Relationships

| Type | Target |
|------|--------|
| related | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
| related | [seed_sidecar_pairs](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/seed_sidecar_pairs.md) |
| related | [pair_sidecar_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pair_sidecar_entities.md) |
| related | [pairing_covers_all_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pairing_covers_all_geometry.md) |
| related | [copy_entity_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/copy_entity_geometry.md) |
