---
okf_version: "0.2"
type: Function
title: deserialize
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/deserialize_1
language: rust
---

# deserialize

## Signature

```rust
fn deserialize(de: D) -> Result<Self, D::Error>
```

## Type Parameters

- `D: serde::Deserializer<'de`

## Source
Lines 224–227 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
