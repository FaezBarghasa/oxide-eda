---
okf_version: "0.2"
type: Function
title: symbol_body_aabb
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/symbol_body_aabb
language: rust
---

# symbol_body_aabb

## Signature

```rust
fn symbol_body_aabb(symbol: &Symbol) -> Aabb
```

## Source
Lines 579–588 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
