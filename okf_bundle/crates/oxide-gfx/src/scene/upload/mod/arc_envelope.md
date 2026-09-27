---
okf_version: "0.2"
type: Function
title: arc_envelope
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod/arc_envelope
language: rust
---

# arc_envelope

## Signature

```rust
fn arc_envelope(arc: &Arc) -> AABB<[f32; 2]>
```

## Source
Lines 122–128 in `crates/oxide-gfx/src/scene/upload/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [upload](/crates/oxide-gfx/src/scene/upload/mod.md) |
| called_by | [apply_dirty_uploads_with_culling](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling.md) |
