---
okf_version: "0.2"
type: Function
title: matches_query
description: "Case-insensitive substring match on the row's label, command id or"
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/matches_query_1
language: rust
---

# matches_query

Case-insensitive substring match on the row's label, command id or

## Signature

```rust
pub fn matches_query(&self, query: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Case-insensitive substring match on the row's label, command id or
current trigger text. An empty (or whitespace-only) query matches
every row. Never panics — safe on empty / odd input.

## Source
Lines 259–273 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |
