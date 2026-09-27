---
okf_version: "0.2"
type: Class
title: NewComponentState
description: "\"New Component\" modal state — collected before the dispatcher"
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/NewComponentState
language: rust
---

# NewComponentState

"New Component" modal state — collected before the dispatcher

## Signature

```rust
pub struct NewComponentState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

"New Component" modal state — collected before the dispatcher
inserts a row into the chosen target table and opens the
Component Preview tab.
[derive(Debug, Clone)]

## Methods

- `internal_pn`
- `library_idx`
- `table`
- `class`
- `category`
- `symbol_ref`
- `footprint_ref`
- `error`
- `creating_table`
- `advanced_open`

## Source
Lines 20–58 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
