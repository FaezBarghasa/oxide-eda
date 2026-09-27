---
okf_version: "0.2"
type: Function
title: list_symbols
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/list_symbols
language: rust
---

# list_symbols

## Signature

```rust
impl AppState { pub fn list_symbols(
        &self,
        library_id: Option<Uuid>,
    ) -> Result<Vec<PrimitiveSummary>, ApiError> }
```

## Visibility

- `pub`

## Source
Lines 326–331 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [list_primitive_summaries](/crates/oxide-library-server/src/db/mod/list_primitive_summaries.md) |
