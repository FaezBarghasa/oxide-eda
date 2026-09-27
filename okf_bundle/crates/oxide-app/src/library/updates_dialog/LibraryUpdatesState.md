---
okf_version: "0.2"
type: Class
title: LibraryUpdatesState
description: "Modal state — owned by [`crate::library::LibraryState::library_updates`]."
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/LibraryUpdatesState
language: rust
---

# LibraryUpdatesState

Modal state — owned by [`crate::library::LibraryState::library_updates`].

## Signature

```rust
pub struct LibraryUpdatesState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Modal state — owned by [`crate::library::LibraryState::library_updates`].
`None` while closed; populated by the schematic-open scan. Sorted
by `ref_des` (lexicographic with natural-numeric tail handling
would be nicer but isn't worth the dep here — `R12` sorts before
`R2` in pure lex; the user can re-sort visually if it bites).
[derive(Debug, Clone)]

## Methods

- `schematic_path`
- `entries`

## Source
Lines 154–162 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
