---
okf_version: "0.2"
type: Class
title: UnresolvedRefs
description: "Outcome of a [`LibrarySet::unresolved_refs`] sweep."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/UnresolvedRefs
language: rust
---

# UnresolvedRefs

Outcome of a [`LibrarySet::unresolved_refs`] sweep.

## Signature

```rust
pub struct UnresolvedRefs
```

## Decorators

- `derive(Debug, Default, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Outcome of a [`LibrarySet::unresolved_refs`] sweep.

The split exists because the two lists need different wording in
the UI: `missing` is a binding the user should fix, `undetermined`
is a library the app failed to read and says nothing about whether
the binding is right.
[derive(Debug, Default, Clone, PartialEq, Eq)]

## Methods

- `missing`
- `undetermined`

## Source
Lines 261–269 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
