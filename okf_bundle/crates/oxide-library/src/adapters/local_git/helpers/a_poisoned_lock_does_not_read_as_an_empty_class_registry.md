---
okf_version: "0.2"
type: Function
title: a_poisoned_lock_does_not_read_as_an_empty_class_registry
description: "A poisoned lock used to read as an empty registry, which is what"
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/a_poisoned_lock_does_not_read_as_an_empty_class_registry
language: rust
---

# a_poisoned_lock_does_not_read_as_an_empty_class_registry

A poisoned lock used to read as an empty registry, which is what

## Signature

```rust
fn a_poisoned_lock_does_not_read_as_an_empty_class_registry()
```

## Decorators

- `test`

## Docstring

A poisoned lock used to read as an empty registry, which is what
a brand-new library looks like — so nothing anywhere in the UI
could tell the two apart.
[test]

## Source
Lines 315–326 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| calls | [library_file_with_classes](/crates/oxide-library/src/adapters/local_git/helpers/library_file_with_classes.md) |
| calls | [poison](/crates/oxide-library/src/adapters/local_git/helpers/poison.md) |
| calls | [classes_or_report](/crates/oxide-library/src/adapters/local_git/helpers/classes_or_report.md) |
