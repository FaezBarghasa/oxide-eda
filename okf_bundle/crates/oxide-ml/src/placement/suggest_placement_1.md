---
okf_version: "0.2"
type: Function
title: suggest_placement
description: Suggests component placement regions using connectivity force layout
resource: crates/oxide-ml/src/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:17:06Z"
concept_id: crates/oxide-ml/src/placement/suggest_placement_1
language: rust
---

# suggest_placement

Suggests component placement regions using connectivity force layout

## Signature

```rust
pub fn suggest_placement(
        &self,
        component_ids: &[u32],
        nets: &[(u32, Vec<u32>)], // (net_id, component_ids)
        current_positions: &HashMap<u32, Point2D>,
    ) -> Vec<PlacementSuggestion>
```

## Visibility

- `pub`

## Docstring

Suggests component placement regions using connectivity force layout

## Source
Lines 25–69 in `crates/oxide-ml/src/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-ml/src/placement.md) |
