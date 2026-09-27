---
okf_version: "0.2"
type: Class
title: PrimitiveSummary
description: Summary record for a primitive (Symbol / Footprint / SimModel) — what the
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/PrimitiveSummary
language: rust
---

# PrimitiveSummary

Summary record for a primitive (Symbol / Footprint / SimModel) — what the

## Signature

```rust
pub struct PrimitiveSummary
```

## Decorators

- `derive(Clone, Debug, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Summary record for a primitive (Symbol / Footprint / SimModel) — what the
`GET /symbols` etc. routes return when listing a library.
[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]

## Methods

- `library_id`
- `uuid`
- `name`

## Source
Lines 32–36 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
