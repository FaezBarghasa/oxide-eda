---
okf_version: "0.2"
type: Function
title: length
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/length
language: rust
---

# length

## Signature

```rust
fn length(a: Vec3) -> f64
```

## Source
Lines 37–39 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [dot](/crates/oxide-physics/src/collision/dot.md) |
| called_by | [normalize](/crates/oxide-physics/src/collision/normalize.md) |
| called_by | [update_simplex_and_direction](/crates/oxide-physics/src/collision/update_simplex_and_direction.md) |
