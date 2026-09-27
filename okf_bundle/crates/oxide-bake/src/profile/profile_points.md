---
okf_version: "0.2"
type: Function
title: profile_points
description: "Deduplicated Points a traced profile owns, in first-seen order."
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/profile_points
language: rust
---

# profile_points

Deduplicated Points a traced profile owns, in first-seen order.

## Signature

```rust
fn profile_points(
    walk: &[ProfileEdge],
    edges: &HashMap<SketchEntityId, &Entity>,
) -> Vec<SketchEntityId>
```

## Docstring

Deduplicated Points a traced profile owns, in first-seen order.

Includes each Arc's `center`: [`edge_endpoints`] deliberately omits
it because it is not a *topology* vertex, but it IS geometry the
profile owns — a translate that skips it deforms the arc.

## Source
Lines 195–214 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
