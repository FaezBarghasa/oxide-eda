---
okf_version: "0.2"
type: Function
title: serialize
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/serialize
language: rust
---

# serialize

## Signature

```rust
impl ColumnType { fn serialize(&self, ser: S) -> Result<S::Ok, S::Error> }
```

## Type Parameters

- `S: serde::Serializer`

## Source
Lines 218–220 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
