---
okf_version: "0.2"
type: Function
title: classes_or_report
description: "Snapshot the component-class registry out of `lock`."
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/classes_or_report
language: rust
---

# classes_or_report

Snapshot the component-class registry out of `lock`.

## Signature

```rust
pub(super) fn classes_or_report(
    lock: &RwLock<LibraryFile>,
    file_path: &Path,
) -> Vec<crate::library_file::ClassEntry>
```

## Visibility

- `pub(super)`

## Docstring

Snapshot the component-class registry out of `lock`.

A poisoned lock used to return `Vec::new()`, which is byte-identical
to a fresh library's empty registry — so the browser sidebar
rendered no class rows and a lookup by key found nothing, with no
signal anywhere that the registry had simply not been read. Poison
is now reported and the registry read through, because the data
behind the flag is still there: a panicking writer is what the
poison flag exists to announce, not to censor.

The sibling `list_tables` maps poison onto
[`LibraryError::Backend`] instead; it can, because its signature
returns a `Result`. `library_classes()` returns a bare `Vec`, so
the log record is the only place the failure can surface.

## Source
Lines 239–256 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [library_classes](/crates/oxide-library/src/adapters/local_git/adapter/library_classes.md) |
| called_by | [a_clean_lock_reads_the_registry_unchanged](/crates/oxide-library/src/adapters/local_git/helpers/a_clean_lock_reads_the_registry_unchanged.md) |
| called_by | [a_poisoned_lock_does_not_read_as_an_empty_class_registry](/crates/oxide-library/src/adapters/local_git/helpers/a_poisoned_lock_does_not_read_as_an_empty_class_registry.md) |
