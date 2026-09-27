---
okf_version: "0.2"
type: Function
title: list_sims
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/list_sims
language: rust
---

# list_sims

## Signature

```rust
impl DatabaseAdapter { fn list_sims(&self) -> Result<Vec<PrimitiveSummary>, LibraryError> }
```

## Source
Lines 507–509 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
