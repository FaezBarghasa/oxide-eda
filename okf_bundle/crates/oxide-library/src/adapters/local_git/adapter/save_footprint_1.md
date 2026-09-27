---
okf_version: "0.2"
type: Function
title: save_footprint
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/save_footprint_1
language: rust
---

# save_footprint

## Signature

```rust
fn save_footprint(&self, fp: Footprint, message: &str) -> Result<(), LibraryError>
```

## Source
Lines 400–407 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [cascade_after_footprint_save](/crates/oxide-library/src/cascade/cascade_after_footprint_save.md) |
