---
okf_version: "0.2"
type: Function
title: containing_library_mut
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/containing_library_mut
language: rust
---

# containing_library_mut

## Signature

```rust
impl LibraryState { pub fn containing_library_mut(&mut self, child_path: &Path) -> Option<&mut OpenLibrary> }
```

## Visibility

- `pub`

## Source
Lines 97–103 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
