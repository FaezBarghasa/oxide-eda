---
okf_version: "0.2"
type: Function
title: hit_test
resource: crates/oxide-app/src/schematic_runtime/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/schematic_runtime/hit_test/hit_test
language: rust
---

# hit_test

## Signature

```rust
pub fn hit_test(
    snapshot: &SchematicRenderSnapshot,
    world_x: f64,
    world_y: f64,
) -> Option<SelectedItem>
```

## Visibility

- `pub`

## Source
Lines 11–18 in `crates/oxide-app/src/schematic_runtime/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test.md) |
| calls | [hit_test_items](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_items.md) |
