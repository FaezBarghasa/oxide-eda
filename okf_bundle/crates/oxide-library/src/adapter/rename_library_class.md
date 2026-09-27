---
okf_version: "0.2"
type: Function
title: rename_library_class
description: "Atomic helper — rename a class. `old_key` must currently"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/rename_library_class
language: rust
---

# rename_library_class

Atomic helper — rename a class. `old_key` must currently

## Signature

```rust
fn rename_library_class(
        &self,
        old_key: &str,
        new_entry: crate::library_file::ClassEntry,
        msg: &str,
    ) -> Result<(), LibraryError>
```

## Docstring

Atomic helper — rename a class. `old_key` must currently
exist; `new_entry.key` must not collide with any *other*
class. Default pattern matches the other helpers.

## Source
Lines 318–343 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| calls | [library_classes](/crates/oxide-library/src/adapter/library_classes.md) |
| calls | [update_library_classes](/crates/oxide-library/src/adapter/update_library_classes.md) |
