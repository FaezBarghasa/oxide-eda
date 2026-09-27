---
okf_version: "0.2"
type: Function
title: try_lock
description: ── Locks (advisory) ────────────────────────────────────────────────
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/try_lock
language: rust
---

# try_lock

── Locks (advisory) ────────────────────────────────────────────────

## Signature

```rust
fn try_lock(&self, _row_id: RowId, _field_set: FieldSet) -> Result<(), LibraryError>
```

## Docstring

── Locks (advisory) ────────────────────────────────────────────────

## Source
Lines 394–398 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
