---
okf_version: "0.2"
type: Function
title: sidecar_id
description: "THE SEEDING RULE, in one place. A `pad.shape_params` VALUE that"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sidecar_id
language: rust
---

# sidecar_id

THE SEEDING RULE, in one place. A `pad.shape_params` VALUE that

## Signature

```rust
fn sidecar_id(value: &str) -> Option<SketchEntityId>
```

## Docstring

THE SEEDING RULE, in one place. A `pad.shape_params` VALUE that
parses as a UUID names one of this pad's own sidecar entities; a
canonical parameter binding (`corner_r` -> `corner_r_<slug>`, see
`helpers::bind_shape_param`) is a parameter NAME, does not parse,
and falls through.

Three readers need it — the move path's sidecar sweep, the delete
path's drop set, and the in-place re-mint's pairing — and their
TRAVERSALS are deliberately different. The seed must not be: a
future shape that records its ids under a new key convention has to
be taught here once, or it gets anchors that translate on a drag
and survive a delete.

## Source
Lines 401–403 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| called_by | [seed_sidecar_pairs](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/seed_sidecar_pairs.md) |
