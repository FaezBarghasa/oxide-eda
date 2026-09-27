---
okf_version: "0.2"
type: Function
title: item_msg
description: Look up a dropdown row by its exact label — robust against the
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/item_msg
language: rust
---

# item_msg

Look up a dropdown row by its exact label — robust against the

## Signature

```rust
fn item_msg(
        entries: &'a [DropdownEntry<LibraryMessage>],
        label: &str,
    ) -> &'a SymbolEditorMsg
```

## Type Parameters

- `'a`

## Docstring

Look up a dropdown row by its exact label — robust against the
row's position shifting as sibling stub rows are wired up.

## Source
Lines 419–441 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
