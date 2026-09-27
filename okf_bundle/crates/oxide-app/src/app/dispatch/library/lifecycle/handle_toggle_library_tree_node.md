---
okf_version: "0.2"
type: Function
title: handle_toggle_library_tree_node
description: "Toggle the Library left-dock panel's library tree node at `idx`."
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_toggle_library_tree_node
language: rust
---

# handle_toggle_library_tree_node

Toggle the Library left-dock panel's library tree node at `idx`.

## Signature

```rust
impl Oxide { pub(super) fn handle_toggle_library_tree_node(&mut self, idx: usize) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Toggle the Library left-dock panel's library tree node at `idx`.

## Source
Lines 209–214 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
