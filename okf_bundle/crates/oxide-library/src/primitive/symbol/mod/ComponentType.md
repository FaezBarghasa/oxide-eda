---
okf_version: "0.2"
type: Class
title: ComponentType
description: "Altium \"Component Type\" — drives BOM rules and schematic"
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/ComponentType
language: rust
---

# ComponentType

Altium "Component Type" — drives BOM rules and schematic

## Signature

```rust
pub enum ComponentType
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Altium "Component Type" — drives BOM rules and schematic
behaviour. `Standard` is the normal electrical component.
`#[non_exhaustive]` because Altium ships a handful of niche types
(Standard No BOM, Net Tie, etc.) — we add as needed.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
[non_exhaustive]

## Source
Lines 302–310 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
