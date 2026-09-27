---
okf_version: "0.2"
type: Module
title: keepout
description: Keepout bake — turns KeepoutAttr-tagged closed profiles into
resource: crates/oxide-bake/src/keepout.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/keepout
language: rust
---

# keepout

Keepout bake — turns KeepoutAttr-tagged closed profiles into

## Docstring

Keepout bake — turns KeepoutAttr-tagged closed profiles into
`Footprint::keepouts: Vec<FpKeepout>` records.

Phase B / Stage 4 of the v0.14.1 sketch-mode plan. v0.14.1 records
the polygon boundary + layer + KeepoutForbid (mapped from the
sketch-side KeepoutKinds bitfield). DRC enforcement is a v0.15
consumer concern.

Mapping `KeepoutKinds` (6 booleans) to `KeepoutForbid` (5
variants):
- More than one bit set → `KeepoutForbid::All`.
- Single bit:
- `no_components` → `Pads`
- `no_routing`    → `Tracks`
- `no_vias`       → `Vias`
- `no_copper`     → `Copper`
- `no_pours`      → `Copper` (closest match in v0.14.1 enum)
- `no_drilling`   → `All` (no dedicated drill-only variant in
v0.14.1 — the lib enum can grow in v0.15+)
- No bits set → `KeepoutForbid::All` (defensive default — an
untyped keepout zone forbids everything).

## Relationships

| Type | Target |
|------|--------|
| related | [bake_keepouts](/crates/oxide-bake/src/keepout/bake_keepouts.md) |
| related | [map_kinds](/crates/oxide-bake/src/keepout/map_kinds.md) |
| related | [solve](/crates/oxide-bake/src/keepout/solve.md) |
| related | [rectangle_with_keepout](/crates/oxide-bake/src/keepout/rectangle_with_keepout.md) |
| related | [bake_keepout_single_kind_routing_maps_to_tracks](/crates/oxide-bake/src/keepout/bake_keepout_single_kind_routing_maps_to_tracks.md) |
| related | [bake_keepout_multiple_kinds_maps_to_all](/crates/oxide-bake/src/keepout/bake_keepout_multiple_kinds_maps_to_all.md) |
| related | [bake_keepout_no_kinds_set_is_all](/crates/oxide-bake/src/keepout/bake_keepout_no_kinds_set_is_all.md) |
| related | [bake_keepout_pours_maps_to_copper](/crates/oxide-bake/src/keepout/bake_keepout_pours_maps_to_copper.md) |
