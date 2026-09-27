---
okf_version: "0.2"
type: Function
title: find_seed_on_plane
resource: crates/oxide-bake/src/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/body3d/find_seed_on_plane
language: rust
---

# find_seed_on_plane

## Signature

```rust
fn find_seed_on_plane(sketch: &SketchData, plane_id: PlaneId) -> Option<SketchEntityId>
```

## Source
Lines 123–133 in `crates/oxide-bake/src/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-bake/src/body3d.md) |
| called_by | [bake_body3d](/crates/oxide-bake/src/body3d/bake_body3d.md) |
