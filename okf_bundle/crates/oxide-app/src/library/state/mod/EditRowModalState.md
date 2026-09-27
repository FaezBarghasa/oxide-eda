---
okf_version: "0.2"
type: Class
title: EditRowModalState
description: "\"Edit Component Details\" modal state — opened by double-clicking a"
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/EditRowModalState
language: rust
---

# EditRowModalState

"Edit Component Details" modal state — opened by double-clicking a

## Signature

```rust
pub struct EditRowModalState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

"Edit Component Details" modal state — opened by double-clicking a
row in the browser grid. The user edits a working copy; commit
fires `adapter.update_row` via `BrowserEditMsg::Save`.
[derive(Debug, Clone)]

## Methods

- `address`
- `draft`
- `param_buf`
- `tags_buf`
- `error`

## Source
Lines 302–316 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
