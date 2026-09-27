---
okf_version: "0.2"
type: Class
title: ProjectTreeContextMenuState
description: "State for the Projects-panel tree-view right-click menu. The menu's"
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/ProjectTreeContextMenuState
language: rust
---

# ProjectTreeContextMenuState

State for the Projects-panel tree-view right-click menu. The menu's

## Signature

```rust
pub struct ProjectTreeContextMenuState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the Projects-panel tree-view right-click menu. The menu's
action set is computed from `path` (leaf vs branch vs empty) at render
time, so we only need to store the anchor coordinates + the clicked
path (or `None` for the background menu).
[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `path`

## Source
Lines 110–116 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
