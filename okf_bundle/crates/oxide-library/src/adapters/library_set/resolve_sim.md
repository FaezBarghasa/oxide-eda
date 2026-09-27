---
okf_version: "0.2"
type: Function
title: resolve_sim
description: "Resolve a `PrimitiveRef` to the underlying [`SimModel`]. Same"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/resolve_sim
language: rust
---

# resolve_sim

Resolve a `PrimitiveRef` to the underlying [`SimModel`]. Same

## Signature

```rust
impl LibrarySet { pub fn resolve_sim(&self, r: &PrimitiveRef) -> Result<Option<SimModel>, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Resolve a `PrimitiveRef` to the underlying [`SimModel`]. Same
`Ok(None)` / `Err` split as [`Self::resolve_symbol`].

## Source
Lines 209–214 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [absence_is_none](/crates/oxide-library/src/adapters/library_set/absence_is_none.md) |
