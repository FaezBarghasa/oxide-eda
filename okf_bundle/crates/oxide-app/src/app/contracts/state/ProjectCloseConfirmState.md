---
okf_version: "0.2"
type: Class
title: ProjectCloseConfirmState
description: "State for the \"Close Project — Unsaved Edits\" confirmation modal."
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/ProjectCloseConfirmState
language: rust
---

# ProjectCloseConfirmState

State for the "Close Project — Unsaved Edits" confirmation modal.

## Signature

```rust
pub struct ProjectCloseConfirmState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the "Close Project — Unsaved Edits" confirmation modal.
Opens only when the user closes a project that has at least one
entry in `DocumentState.dirty_paths` rooted in the project's
directory; the modal lists every dirty file by filename so the
user can see what they're about to lose.
[derive(Debug, Clone)]

## Methods

- `tree_path`
- `project_name`
- `dirty_paths`

## Source
Lines 124–138 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
