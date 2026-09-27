---
okf_version: "0.2"
type: Function
title: remove_library_class
description: Atomic helper — remove a class by key. Default goes through
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/remove_library_class
language: rust
---

# remove_library_class

Atomic helper — remove a class by key. Default goes through

## Signature

```rust
fn remove_library_class(&self, key: &str, msg: &str) -> Result<(), LibraryError>
```

## Docstring

Atomic helper — remove a class by key. Default goes through
the same two-step pattern as `add_library_class`. No-op when
the key isn't present.

## Source
Lines 305–313 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| calls | [library_classes](/crates/oxide-library/src/adapter/library_classes.md) |
| calls | [update_library_classes](/crates/oxide-library/src/adapter/update_library_classes.md) |
