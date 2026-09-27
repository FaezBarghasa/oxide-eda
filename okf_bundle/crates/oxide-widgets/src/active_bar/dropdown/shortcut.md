---
okf_version: "0.2"
type: Function
title: shortcut
description: "Builder: attach a right-aligned shortcut hint."
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown/shortcut
language: rust
---

# shortcut

Builder: attach a right-aligned shortcut hint.

## Signature

```rust
impl DropdownItem<M> { pub fn shortcut(mut self, hint: impl Into<String>) -> Self }
```

## Type Parameters

- `M: 'static + Clone`

## Visibility

- `pub`

## Docstring

Builder: attach a right-aligned shortcut hint.

## Source
Lines 118–121 in `crates/oxide-widgets/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-widgets/src/active_bar/dropdown.md) |
