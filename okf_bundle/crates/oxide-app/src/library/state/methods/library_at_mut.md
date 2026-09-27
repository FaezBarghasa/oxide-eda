---
okf_version: "0.2"
type: Function
title: library_at_mut
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/library_at_mut
language: rust
---

# library_at_mut

## Signature

```rust
impl LibraryState { pub fn library_at_mut(&mut self, path: &Path) -> Option<&mut OpenLibrary> }
```

## Visibility

- `pub`

## Source
Lines 75–77 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
