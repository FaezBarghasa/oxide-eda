---
okf_version: "0.2"
type: Function
title: library_classes
description: Per-library class registry — the source of truth for the New
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/library_classes
language: rust
---

# library_classes

Per-library class registry — the source of truth for the New

## Signature

```rust
fn library_classes(&self) -> Vec<crate::library_file::ClassEntry>
```

## Docstring

Per-library class registry — the source of truth for the New
Component / Edit Component class dropdowns. Adapters that
don't carry a manifest return an empty list.

# Hazard for implementors

The signature has no way to say "I could not read the
registry", so an empty return is indistinguishable from a fresh
library with no classes yet. That matters more than it looks,
because [`Self::add_library_class`],
[`Self::remove_library_class`] and
[`Self::rename_library_class`] all default to a read-modify-write
round-trip over this method: an implementation that answers
`Vec::new()` on a read failure and then inherits those defaults
would write the empty list straight back and **erase the
registry on disk**.

`LocalGitAdapter` is safe on both counts — it overrides all
three mutators with single-borrow read-modify-writes, and its
`library_classes` reports a read failure instead of substituting
an empty list. A new adapter must do one or the other before
relying on the defaults.

## Source
Lines 260–262 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| called_by | [LibraryAdapter](/crates/oxide-library/src/adapter/LibraryAdapter.md) |
| called_by | [add_library_class](/crates/oxide-library/src/adapter/add_library_class.md) |
| called_by | [remove_library_class](/crates/oxide-library/src/adapter/remove_library_class.md) |
| called_by | [rename_library_class](/crates/oxide-library/src/adapter/rename_library_class.md) |
