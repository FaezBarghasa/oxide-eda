---
okf_version: "0.2"
type: Function
title: new_core
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/new_core
language: rust
---

# new_core

## Signature

```rust
impl LayerDefinition { pub fn new_core(
        name: impl Into<String>,
        thickness: Microns,
        material: MaterialProperties,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 107–119 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
