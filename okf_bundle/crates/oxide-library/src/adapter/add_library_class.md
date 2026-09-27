---
okf_version: "0.2"
type: Function
title: add_library_class
description: Atomic helper — append a class. The default implementation
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/add_library_class
language: rust
---

# add_library_class

Atomic helper — append a class. The default implementation

## Signature

```rust
fn add_library_class(
        &self,
        entry: crate::library_file::ClassEntry,
        msg: &str,
    ) -> Result<(), LibraryError>
```

## Docstring

Atomic helper — append a class. The default implementation
is a `library_classes` + `update_library_classes` round-trip
(which acquires the lock twice and can't be made atomic
without backend support). Adapters that own the manifest
in-memory (e.g. `LocalGitAdapter` via `mutate_library_file`)
should override with a single-borrow read-modify-write.
Errors `Conflict` when a class with the same key already
exists.

## Source
Lines 286–300 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| calls | [library_classes](/crates/oxide-library/src/adapter/library_classes.md) |
| calls | [update_library_classes](/crates/oxide-library/src/adapter/update_library_classes.md) |
