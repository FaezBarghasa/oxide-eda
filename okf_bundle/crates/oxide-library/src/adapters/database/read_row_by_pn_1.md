---
okf_version: "0.2"
type: Function
title: read_row_by_pn
description: "Linear scan via `iter_rows` — same composition rationale as"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/read_row_by_pn_1
language: rust
---

# read_row_by_pn

Linear scan via `iter_rows` — same composition rationale as

## Signature

```rust
fn read_row_by_pn(&self, pn: &InternalPn) -> Result<(String, ComponentRow), LibraryError>
```

## Docstring

Linear scan via `iter_rows` — same composition rationale as
`iter_rows`. The server has no PN index endpoint; v0.9 acceptable.

## Source
Lines 368–375 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
