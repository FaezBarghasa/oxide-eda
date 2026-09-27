---
okf_version: "0.2"
type: Class
title: RenameDialogState
description: "State for the rename modal. Tracks the target file, the live"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/RenameDialogState
language: rust
---

# RenameDialogState

State for the rename modal. Tracks the target file, the live

## Signature

```rust
pub struct RenameDialogState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the rename modal. Tracks the target file, the live
edit buffer, and the clicked tree path so we can rebuild the tree
after a successful rename without rediscovering the project.
[derive(Debug, Clone)]

## Methods

- `target_path`
- `tree_path`
- `buffer`
- `error`
- `is_project_rename`

## Source
Lines 275–286 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
