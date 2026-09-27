---
okf_version: "0.2"
type: Function
title: save_sim
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/save_sim
language: rust
---

# save_sim

## Signature

```rust
impl DatabaseAdapter { fn save_sim(&self, sm: SimModel, message: &str) -> Result<(), LibraryError> }
```

## Source
Lines 495–497 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
