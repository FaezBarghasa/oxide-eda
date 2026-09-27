---
okf_version: "0.2"
type: Class
title: SymbolGraphicKind
description: Drawing primitive kinds — the geometry of the symbol body.
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/SymbolGraphicKind
language: rust
---

# SymbolGraphicKind

Drawing primitive kinds — the geometry of the symbol body.

## Signature

```rust
pub enum SymbolGraphicKind
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Drawing primitive kinds — the geometry of the symbol body.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "snake_case")]

## Methods

- `from`
- `to`
- `from`
- `to`
- `center`
- `radius`
- `center`
- `radius`
- `start_deg`
- `end_deg`
- `position`
- `content`
- `size`
- `vertices`

## Source
Lines 200–232 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
