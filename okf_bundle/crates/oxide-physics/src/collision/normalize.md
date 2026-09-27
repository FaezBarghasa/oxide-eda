---
okf_version: "0.2"
type: Function
title: normalize
resource: crates/oxide-physics/src/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:37:16Z"
concept_id: crates/oxide-physics/src/collision/normalize
language: rust
---

# normalize

## Signature

```rust
fn normalize(a: Vec3) -> Vec3
```

## Source
Lines 41–44 in `crates/oxide-physics/src/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-physics/src/collision.md) |
| calls | [length](/crates/oxide-physics/src/collision/length.md) |
| calls | [scale](/crates/oxide-physics/src/collision/scale.md) |
| called_by | [evaluate_collision](/crates/oxide-physics/src/collision/evaluate_collision.md) |
