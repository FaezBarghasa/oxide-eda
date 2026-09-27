---
okf_version: "0.2"
type: Function
title: read_primitive
description: "Read a primitive JSON file at `<root>/<subdir>/<uuid>.<ext>`."
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/read_primitive_1
language: rust
---

# read_primitive

Read a primitive JSON file at `<root>/<subdir>/<uuid>.<ext>`.

## Signature

```rust
pub(super) fn read_primitive(
        &self,
        kind: PrimitiveKind,
        uuid: Uuid,
    ) -> Result<T, LibraryError>
```

## Type Parameters

- `T: DeserializeOwned`

## Visibility

- `pub(super)`

## Docstring

Read a primitive JSON file at `<root>/<subdir>/<uuid>.<ext>`.

## Source
Lines 17–77 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
