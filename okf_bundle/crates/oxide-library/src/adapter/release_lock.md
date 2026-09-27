---
okf_version: "0.2"
type: Function
title: release_lock
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/release_lock
language: rust
---

# release_lock

## Signature

```rust
fn release_lock(&self, _row_id: RowId, _field_set: FieldSet) -> Result<(), LibraryError>
```

## Source
Lines 400–404 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
