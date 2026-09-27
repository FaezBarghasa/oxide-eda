---
okf_version: "0.2"
type: Class
title: WindowKind
description: Role of a non-main window opened by Oxide. Phase 2 adds detached
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/WindowKind
language: rust
---

# WindowKind

Role of a non-main window opened by Oxide. Phase 2 adds detached

## Signature

```rust
pub enum WindowKind
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Role of a non-main window opened by Oxide. Phase 2 adds detached
modals; Phase 3 adds `UndockedTab(tab_index)` so a schematic sheet
can live in its own OS window.
[derive(Debug, Clone)]

## Methods

- `path`
- `title`
- `library_path`
- `table`
- `row_id`

## Source
Lines 83–105 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
