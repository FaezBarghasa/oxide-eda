---
okf_version: "0.2"
type: Function
title: nonempty_bucket_sequence
description: "The buckets of `order`, in the order they appear, restricted to the ones"
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/nonempty_bucket_sequence
language: rust
---

# nonempty_bucket_sequence

The buckets of `order`, in the order they appear, restricted to the ones

## Signature

```rust
fn nonempty_bucket_sequence(scene: &Scene, order: &[SceneBucket]) -> Vec<SceneBucket>
```

## Docstring

The buckets of `order`, in the order they appear, restricted to the ones
this concrete scene actually has geometry in -- the real, drawn paint
sequence rather than the abstract bucket-name list.

## Source
Lines 356–362 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| calls | [bucket_count](/crates/oxide-gfx/src/scene/scenario_tests/bucket_count.md) |
| called_by | [overlays_composite_above_base_content_in_a_fully_populated_scene](/crates/oxide-gfx/src/scene/scenario_tests/overlays_composite_above_base_content_in_a_fully_populated_scene.md) |
