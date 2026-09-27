---
okf_version: "0.2"
type: Function
title: cull_items
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod/cull_items
language: rust
---

# cull_items

## Signature

```rust
fn cull_items(
    items: &'a [T],
    viewport: Option<&AABB<[f32; 2]>>,
    envelope_of: impl Fn(&T) -> Option<AABB<[f32; 2]>>,
) -> Cow<'a, [T]>
```

## Type Parameters

- `'a`
- `T: Clone`

## Source
Lines 164–203 in `crates/oxide-gfx/src/scene/upload/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [upload](/crates/oxide-gfx/src/scene/upload/mod.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| called_by | [apply_dirty_uploads_with_culling](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling.md) |
