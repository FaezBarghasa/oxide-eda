---
okf_version: "0.2"
type: Function
title: new
description: Build a state from a freshly-collected drift list. Auto-sorts
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/new_1
language: rust
---

# new

Build a state from a freshly-collected drift list. Auto-sorts

## Signature

```rust
pub fn new(schematic_path: PathBuf, mut entries: Vec<LibraryUpdateEntry>) -> Self
```

## Visibility

- `pub`

## Docstring

Build a state from a freshly-collected drift list. Auto-sorts
by `ref_des` and applies the `bump_kind.default_checked()`
rule to each entry's checkbox.

## Source
Lines 168–177 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
| calls | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
