---
okf_version: "0.2"
type: Function
title: save_sim
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/save_sim_1
language: rust
---

# save_sim

## Signature

```rust
fn save_sim(&self, sm: SimModel, message: &str) -> Result<(), LibraryError>
```

## Source
Lines 409–416 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [cascade_after_sim_save](/crates/oxide-library/src/cascade/cascade_after_sim_save.md) |
