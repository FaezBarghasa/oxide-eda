---
okf_version: "0.2"
type: Function
title: insert_symbol
description: ── Primitive CRUD ────────────────────────────────────────────────────
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/insert_symbol
language: rust
---

# insert_symbol

── Primitive CRUD ────────────────────────────────────────────────────

## Signature

```rust
impl AppState { pub fn insert_symbol(&self, library_id: Uuid, sym: &Symbol) -> Result<(), ApiError> }
```

## Visibility

- `pub`

## Docstring

── Primitive CRUD ────────────────────────────────────────────────────

## Source
Lines 314–317 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [upsert_primitive](/crates/oxide-library-server/src/db/mod/upsert_primitive.md) |
