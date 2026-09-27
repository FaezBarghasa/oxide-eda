---
okf_version: "0.2"
type: Function
title: write_primitive
description: "Persist a primitive file under `<root>/<subdir>/<uuid>.<ext>`,"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/write_primitive
language: rust
---

# write_primitive

Persist a primitive file under `<root>/<subdir>/<uuid>.<ext>`,

## Signature

```rust
impl LocalGitAdapter { pub(super) fn write_primitive(
        &self,
        kind: PrimitiveKind,
        uuid: Uuid,
        value: &T,
        message: &str,
    ) -> Result<(), LibraryError> }
```

## Type Parameters

- `T: Serialize`

## Visibility

- `pub(super)`

## Docstring

Persist a primitive file under `<root>/<subdir>/<uuid>.<ext>`,
stage + commit it via libgit2 with the supplied message.

`.snxfpt` files emit as TOML+TSV envelope (v0.18.4); `.snxsim`
files emit as TOML envelope (v0.18.5). `.snxsym` is handled
outside this generic path via `save_symbol_in_container` so
multi-symbol containers are preserved.

## Source
Lines 86–174 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
