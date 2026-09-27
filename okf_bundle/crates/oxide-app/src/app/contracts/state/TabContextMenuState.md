---
okf_version: "0.2"
type: Class
title: TabContextMenuState
description: "State for the document-tab right-click menu. The menu's items are"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/TabContextMenuState
language: rust
---

# TabContextMenuState

State for the document-tab right-click menu. The menu's items are

## Signature

```rust
pub struct TabContextMenuState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the document-tab right-click menu. The menu's items are
derived from `tab_idx` (the clicked tab) so the same menu builder
works for any tab; mutually exclusive with the canvas and project-
tree context menus.
[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `tab_idx`

## Source
Lines 170–174 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
