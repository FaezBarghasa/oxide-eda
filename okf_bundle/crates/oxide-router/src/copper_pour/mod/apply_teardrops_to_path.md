---
okf_version: "0.2"
type: Function
title: apply_teardrops_to_path
description: Add teardrops across all vias in a routed path.
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod/apply_teardrops_to_path
language: rust
---

# apply_teardrops_to_path

Add teardrops across all vias in a routed path.

## Signature

```rust
impl TeardropGenerator { pub fn apply_teardrops_to_path(
        segments: &[RouteSegment],
        vias: &[ViaPlacement],
    ) -> Vec<RouteSegment> }
```

## Visibility

- `pub`

## Docstring

Add teardrops across all vias in a routed path.

## Source
Lines 188–226 in `crates/oxide-router/src/copper_pour/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copper_pour](/crates/oxide-router/src/copper_pour/mod.md) |
