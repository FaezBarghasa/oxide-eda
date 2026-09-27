---
okf_version: "0.2"
type: Function
title: fetch_symbol
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/fetch_symbol
language: rust
---

# fetch_symbol

## Signature

```rust
impl AppState { pub fn fetch_symbol(&self, library_id: Uuid, uuid: Uuid) -> Result<Option<Symbol>, ApiError> }
```

## Visibility

- `pub`

## Source
Lines 319–324 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [fetch_primitive_payload](/crates/oxide-library-server/src/db/mod/fetch_primitive_payload.md) |
