---
okf_version: "0.2"
type: Class
title: RemoveDialogState
description: "State for the \"Remove from Project\" modal. `Delete` removes the file"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/RemoveDialogState
language: rust
---

# RemoveDialogState

State for the "Remove from Project" modal. `Delete` removes the file

## Signature

```rust
pub struct RemoveDialogState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the "Remove from Project" modal. `Delete` removes the file
from disk; `Exclude` drops it from the session's sheet list but
leaves the file in place.
[derive(Debug, Clone)]

## Methods

- `target_path`
- `tree_path`
- `display_name`

## Source
Lines 376–380 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
