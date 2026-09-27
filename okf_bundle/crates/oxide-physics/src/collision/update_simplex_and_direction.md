---
okf_version: "0.2"
type: Function
title: update_simplex_and_direction
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/update_simplex_and_direction
language: rust
---

# update_simplex_and_direction

## Signature

```rust
impl GjkEpaEngine { fn update_simplex_and_direction(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool }
```

## Source
Lines 139–177 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [sub](/crates/oxide-physics/src/collision/sub.md) |
| calls | [scale](/crates/oxide-physics/src/collision/scale.md) |
| calls | [cross](/crates/oxide-physics/src/collision/cross.md) |
| calls | [length](/crates/oxide-physics/src/collision/length.md) |
| calls | [dot](/crates/oxide-physics/src/collision/dot.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
