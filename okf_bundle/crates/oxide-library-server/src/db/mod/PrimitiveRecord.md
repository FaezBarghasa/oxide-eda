---
okf_version: "0.2"
type: Class
title: PrimitiveRecord
description: "[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/PrimitiveRecord
language: rust
---

# PrimitiveRecord

[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]

## Signature

```rust
pub struct PrimitiveRecord
```

## Decorators

- `derive(Clone, Debug, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]

## Methods

- `library_id`
- `uuid`
- `name`
- `payload`
- `created_at`
- `updated_at`

## Source
Lines 50–57 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
