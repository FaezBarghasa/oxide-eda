---
okf_version: "0.2"
type: Class
title: DropdownEntry
description: One row inside the dropdown. The widget walks the slice in order
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown/DropdownEntry
language: rust
---

# DropdownEntry

One row inside the dropdown. The widget walks the slice in order

## Signature

```rust
pub enum DropdownEntry
```

## Type Parameters

- `M`

## Visibility

- `pub`

## Docstring

One row inside the dropdown. The widget walks the slice in order
so semantic grouping (section header → items → separator → next
section) reads top-down at the call site.

## Source
Lines 54–71 in `crates/oxide-widgets/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-widgets/src/active_bar/dropdown.md) |
