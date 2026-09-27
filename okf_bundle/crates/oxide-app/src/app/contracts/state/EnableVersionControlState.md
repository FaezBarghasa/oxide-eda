---
okf_version: "0.2"
type: Class
title: EnableVersionControlState
description: "State for the \"Enable Version Control\" confirm modal — opened"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/EnableVersionControlState
language: rust
---

# EnableVersionControlState

State for the "Enable Version Control" confirm modal — opened

## Signature

```rust
pub struct EnableVersionControlState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the "Enable Version Control" confirm modal — opened
from the project root context menu when the project directory
has no `.git/` yet, or from a plain-files `.snxlib` node's
right-click menu. Confirm runs `git2::Repository::init` at
`project_dir`, optionally writes `.gitattributes` for binary-
model LFS, generates a `.gitignore` from the unticked items,
and stages an initial commit covering the picked subset.
[derive(Debug, Clone)]

## Methods

- `scope`
- `project_path`
- `project_dir`
- `project_name`
- `items`
- `use_lfs`
- `intro_text`
- `error`

## Source
Lines 324–356 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
