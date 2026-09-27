---
okf_version: "0.2"
type: Function
title: list_primitives_json
description: "Generic GET → list at `/{collection}` returning [`PrimitiveSummary`]."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/list_primitives_json
language: rust
---

# list_primitives_json

Generic GET → list at `/{collection}` returning [`PrimitiveSummary`].

## Signature

```rust
impl DatabaseAdapter { fn list_primitives_json(
        &self,
        collection: &str,
    ) -> Result<Vec<PrimitiveSummary>, LibraryError> }
```

## Docstring

Generic GET → list at `/{collection}` returning [`PrimitiveSummary`].

## Source
Lines 241–258 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
