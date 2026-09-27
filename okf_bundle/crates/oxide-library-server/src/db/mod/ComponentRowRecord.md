---
okf_version: "0.2"
type: Class
title: ComponentRowRecord
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
concept_id: crates/oxide-library-server/src/db/mod/ComponentRowRecord
language: rust
---

# ComponentRowRecord

[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]

## Signature

```rust
pub struct ComponentRowRecord
```

## Decorators

- `derive(Clone, Debug, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]

## Methods

- `library_id`
- `table_name`
- `row_id`
- `internal_pn`
- `payload`
- `created_at`
- `updated_at`

## Source
Lines 39–47 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
