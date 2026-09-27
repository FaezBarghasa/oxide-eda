---
okf_version: "0.2"
type: Function
title: read_row_by_pn
description: Linear scan across every table — O(total rows). Acceptable at
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/read_row_by_pn_1
language: rust
---

# read_row_by_pn

Linear scan across every table — O(total rows). Acceptable at

## Signature

```rust
fn read_row_by_pn(&self, pn: &InternalPn) -> Result<(String, ComponentRow), LibraryError>
```

## Docstring

Linear scan across every table — O(total rows). Acceptable at
v0.9 scale (libraries are O(thousands)). When the search index
lands the call should redirect through it.

## Source
Lines 281–288 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
