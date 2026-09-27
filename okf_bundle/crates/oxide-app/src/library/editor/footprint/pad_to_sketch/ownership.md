---
okf_version: "0.2"
type: Module
title: ownership
description: Which sketch entities a pad OWNS.
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership
language: rust
---

# ownership

Which sketch entities a pad OWNS.

## Docstring

Which sketch entities a pad OWNS.

The DURABLE answer is [`oxide_sketch::attr::PadAttr::owned`], a
ledger written onto the centre Point's `PadAttr` at mint time by
[`record_ledger`]. It is the only one of the four ownership records
that survives a save + reopen.

The other three live on `EditorPad` and are session-volatile —
`EditorPad::from_pad` sets all of them to `None` / empty because
they have no home on `Pad`:
- `sketch_entity_id` — the centre `Point` carrying the `PadAttr`.
- `corner_entity_ids` — the four bbox-corner Points.
- `shape_params` — the per-shape sidecar ledger. RoundRect records
its four corner Arcs (`corner_r_*_arc`), Oval its anchors /
arc-centres / Lines / Arcs (`oval_*`), Chamfered its per-corner
anchors (`chamfer_*_anchor*`).

They are still consulted, because a pad written before the durable
ledger existed has an empty `PadAttr::owned` and the volatile fields
are all it has within the minting session.

Any operation that acts on "the pad's geometry" has to consult all
four. Handling a subset is what stranded RoundRect anchors on a
move and leaked constraints / parameters on a delete — this module
exists so move and delete cannot drift apart again.

## Relationships

| Type | Target |
|------|--------|
| related | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| related | [persisted_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/persisted_ledger.md) |
| related | [record_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger.md) |
