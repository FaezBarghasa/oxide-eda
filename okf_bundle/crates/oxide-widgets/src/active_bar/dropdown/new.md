---
okf_version: "0.2"
type: Function
title: new
description: "Convenience: simple label + on_press, no icon / shortcut."
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown/new
language: rust
---

# new

Convenience: simple label + on_press, no icon / shortcut.

## Signature

```rust
impl DropdownItem<M> { pub fn new(label: impl Into<String>, on_press: M) -> Self }
```

## Type Parameters

- `M: 'static + Clone`

## Visibility

- `pub`

## Docstring

Convenience: simple label + on_press, no icon / shortcut.

## Source
Lines 94–103 in `crates/oxide-widgets/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-widgets/src/active_bar/dropdown.md) |
