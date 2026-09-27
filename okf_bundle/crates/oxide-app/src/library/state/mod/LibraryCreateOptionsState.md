---
okf_version: "0.2"
type: Class
title: LibraryCreateOptionsState
description: "State for the \"Library Options\" modal that pops up between the"
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/LibraryCreateOptionsState
language: rust
---

# LibraryCreateOptionsState

State for the "Library Options" modal that pops up between the

## Signature

```rust
pub struct LibraryCreateOptionsState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the "Library Options" modal that pops up between the
Save-As dialog (where the user picked the `.snxlib` filename) and
the actual `LocalGitAdapter::init` call. Carries the project the
library should attach to + the chosen `.snxlib` path so the
dispatcher can finalise `commands::create_library_at` once the user
confirms.

Two user-facing toggles: `enable_git` (opt-in to version control
— default off, fresh libraries land as plain files) and `use_lfs`
(only meaningful when version control is on).
[derive(Debug, Clone)]

## Methods

- `project_path`
- `lib_path`
- `enable_git`
- `use_lfs`

## Source
Lines 585–602 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
