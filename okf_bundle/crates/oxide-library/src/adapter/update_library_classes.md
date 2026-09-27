---
okf_version: "0.2"
type: Function
title: update_library_classes
description: Persist the class registry. The UI calls this from the
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/update_library_classes
language: rust
---

# update_library_classes

Persist the class registry. The UI calls this from the

## Signature

```rust
fn update_library_classes(
        &self,
        _classes: Vec<crate::library_file::ClassEntry>,
        _msg: &str,
    ) -> Result<(), LibraryError>
```

## Docstring

Persist the class registry. The UI calls this from the
Library Properties pane (Add / Rename / Delete class). Default
implementation surfaces a `Backend` error so adapters that
don't support manifest mutation are detectable.

## Source
Lines 268–276 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| called_by | [LibraryAdapter](/crates/oxide-library/src/adapter/LibraryAdapter.md) |
| called_by | [add_library_class](/crates/oxide-library/src/adapter/add_library_class.md) |
| called_by | [remove_library_class](/crates/oxide-library/src/adapter/remove_library_class.md) |
| called_by | [rename_library_class](/crates/oxide-library/src/adapter/rename_library_class.md) |
