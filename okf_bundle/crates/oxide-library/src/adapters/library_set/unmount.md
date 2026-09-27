---
okf_version: "0.2"
type: Function
title: unmount
description: "Unmount the first adapter matching `library_id` and return it"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/unmount
language: rust
---

# unmount

Unmount the first adapter matching `library_id` and return it

## Signature

```rust
impl LibrarySet { pub fn unmount(&mut self, library_id: Uuid) -> Option<Box<dyn LibraryAdapter>> }
```

## Visibility

- `pub`

## Docstring

Unmount the first adapter matching `library_id` and return it
(or `None` if no adapter is registered under that id).

When two file-backed adapters share an id, the "first match"
is unspecified — callers needing a specific mount should use
[`Self::unmount_by_path`].

## Source
Lines 128–131 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
