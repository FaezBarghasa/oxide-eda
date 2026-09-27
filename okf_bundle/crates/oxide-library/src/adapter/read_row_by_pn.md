---
okf_version: "0.2"
type: Function
title: read_row_by_pn
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/read_row_by_pn
language: rust
---

# read_row_by_pn

## Signature

```rust
fn read_row_by_pn(&self, _pn: &InternalPn) -> Result<(String, ComponentRow), LibraryError>
```

## Source
Lines 368–372 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
