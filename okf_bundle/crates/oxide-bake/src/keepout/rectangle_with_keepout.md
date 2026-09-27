---
okf_version: "0.2"
type: Function
title: rectangle_with_keepout
resource: crates/oxide-bake/src/keepout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/keepout/rectangle_with_keepout
language: rust
---

# rectangle_with_keepout

## Signature

```rust
fn rectangle_with_keepout(attr: KeepoutAttr) -> SketchData
```

## Source
Lines 125–167 in `crates/oxide-bake/src/keepout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keepout](/crates/oxide-bake/src/keepout.md) |
| called_by | [bake_keepout_multiple_kinds_maps_to_all](/crates/oxide-bake/src/keepout/bake_keepout_multiple_kinds_maps_to_all.md) |
| called_by | [bake_keepout_no_kinds_set_is_all](/crates/oxide-bake/src/keepout/bake_keepout_no_kinds_set_is_all.md) |
| called_by | [bake_keepout_pours_maps_to_copper](/crates/oxide-bake/src/keepout/bake_keepout_pours_maps_to_copper.md) |
| called_by | [bake_keepout_single_kind_routing_maps_to_tracks](/crates/oxide-bake/src/keepout/bake_keepout_single_kind_routing_maps_to_tracks.md) |
