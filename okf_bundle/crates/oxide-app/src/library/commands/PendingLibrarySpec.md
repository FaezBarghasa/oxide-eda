---
okf_version: "0.2"
type: Class
title: PendingLibrarySpec
description: "Captured shape of a New Library request that hasn't been written"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/PendingLibrarySpec
language: rust
---

# PendingLibrarySpec

Captured shape of a New Library request that hasn't been written

## Signature

```rust
pub struct PendingLibrarySpec
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Captured shape of a New Library request that hasn't been written
to disk yet. Lives on `LoadedProject.pending_libraries` between
the user clicking "Create Library" on the Library Options modal
and the next successful project save (which materialises the
`.snxlib` via `materialize_pending_library`). Closes
`feedback_no_disk_writes_without_user_save.md`'s "wait for
explicit user save" invariant — modal confirm flips the project
dirty bit but leaves disk untouched; only `Ctrl+S` actually
writes anything.
[derive(Debug, Clone)]

## Methods

- `lib_path`
- `enable_git`
- `use_lfs`
- `display_name`

## Source
Lines 71–79 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
