---
okf_version: "0.2"
type: Class
title: ComponentEditorTab
description: "v0.9-refactor-2: DBLib model. Identity payload for a Component"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/ComponentEditorTab
language: rust
---

# ComponentEditorTab

v0.9-refactor-2: DBLib model. Identity payload for a Component

## Signature

```rust
pub struct ComponentEditorTab
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.9-refactor-2: DBLib model. Identity payload for a Component
Preview tab — `(library_path, table, row_id)` triple from
`EditorAddress`. The inline tab and any future undock case route
through the same triple.
[derive(Debug, Clone)]

## Methods

- `library_path`
- `table`
- `row_id`

## Source
Lines 12–16 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
