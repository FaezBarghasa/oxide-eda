---
okf_version: "0.2"
type: Function
title: aabb_overlaps
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/aabb_overlaps
language: rust
---

# aabb_overlaps

## Signature

```rust
fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool
```

## Source
Lines 703–705 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [hit_test_rect_mode](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_rect_mode.md) |
