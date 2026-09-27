---
okf_version: "0.2"
type: Function
title: persist_library_file
description: "Persist the in-memory `library_file` to disk. Caller already holds"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/persist_library_file_1
language: rust
---

# persist_library_file

Persist the in-memory `library_file` to disk. Caller already holds

## Signature

```rust
fn persist_library_file(&self, lf: &LibraryFile) -> Result<(), LibraryError>
```

## Docstring

Persist the in-memory `library_file` to disk. Caller already holds
the appropriate read/write lock.

## Source
Lines 250–255 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
