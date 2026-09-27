---
okf_version: "0.2"
type: Function
title: resolve_symbol
description: "Resolve a `PrimitiveRef` to the underlying [`Symbol`]."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/resolve_symbol_1
language: rust
---

# resolve_symbol

Resolve a `PrimitiveRef` to the underlying [`Symbol`].

## Signature

```rust
pub fn resolve_symbol(&self, r: &PrimitiveRef) -> Result<Option<Symbol>, LibraryError>
```

## Visibility

- `pub`

## Docstring

Resolve a `PrimitiveRef` to the underlying [`Symbol`].

`Ok(None)` means the library isn't mounted or the UUID isn't in
it. `Err` means the lookup itself failed and the binding is
**not** known to be wrong — see the module docs.

## Source
Lines 191–196 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [absence_is_none](/crates/oxide-library/src/adapters/library_set/absence_is_none.md) |
