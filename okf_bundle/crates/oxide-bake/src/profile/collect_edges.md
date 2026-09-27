---
okf_version: "0.2"
type: Function
title: collect_edges
description: Collect non-construction edge entities (Lines + Arcs). Circles are
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/collect_edges
language: rust
---

# collect_edges

Collect non-construction edge entities (Lines + Arcs). Circles are

## Signature

```rust
fn collect_edges(sketch: &SketchData) -> HashMap<SketchEntityId, &Entity>
```

## Docstring

Collect non-construction edge entities (Lines + Arcs). Circles are
excluded — they're already-closed primitives that the bake module
handles separately.

## Source
Lines 341–352 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
