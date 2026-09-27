---
okf_version: "0.2"
type: Function
title: project_for_path
description: Resolve the project that contains a file at this path. Used for
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/project_for_path_1
language: rust
---

# project_for_path

Resolve the project that contains a file at this path. Used for

## Signature

```rust
pub fn project_for_path(&self, path: &std::path::Path) -> Option<&LoadedProject>
```

## Visibility

- `pub`

## Docstring

Resolve the project that contains a file at this path. Used for
per-tab project scoping (tabs store a path, we resolve to the
project that parented them at load time).

## Source
Lines 648–651 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
