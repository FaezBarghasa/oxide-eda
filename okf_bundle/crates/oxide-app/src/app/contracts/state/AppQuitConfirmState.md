---
okf_version: "0.2"
type: Class
title: AppQuitConfirmState
description: "State for the \"Exit Oxide — Unsaved Edits\" confirmation modal."
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/AppQuitConfirmState
language: rust
---

# AppQuitConfirmState

State for the "Exit Oxide — Unsaved Edits" confirmation modal.

## Signature

```rust
pub struct AppQuitConfirmState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the "Exit Oxide — Unsaved Edits" confirmation modal.
Opens when the user requests app exit (chrome ✕, File ▸ Exit,
Alt+F4) while `DocumentState.dirty_paths` is non-empty. Lists
every dirty file across the whole workspace so the user sees what
they are about to lose before choosing Save All / Discard All /
Cancel. Reuses `ProjectCloseChoice` for the three outcomes.
[derive(Debug, Clone)]

## Methods

- `dirty_paths`

## Source
Lines 147–151 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
