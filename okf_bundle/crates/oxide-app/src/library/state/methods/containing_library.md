---
okf_version: "0.2"
type: Function
title: containing_library
description: "Find the open library whose `root_dir` is an ancestor of"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/containing_library
language: rust
---

# containing_library

Find the open library whose `root_dir` is an ancestor of

## Signature

```rust
impl LibraryState { pub fn containing_library(&self, child_path: &Path) -> Option<&OpenLibrary> }
```

## Visibility

- `pub`

## Docstring

Find the open library whose `root_dir` is an ancestor of
`child_path`. Used to resolve `.snxsym` / `.snxfpt` files back
to the `.snxlib` they live alongside — e.g. for sourcing
per-library canvas display settings
([`LibraryDisplaySettings`]).

Per `v0.9-snxlib-as-file-plan.md` §2 Stage C the comparison
is against the `.snxlib`'s *parent directory*, not the file
itself, so `<root_dir>/symbols/foo.snxsym` correctly resolves
to its library.

## Source
Lines 89–95 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
