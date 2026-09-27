---
okf_version: "0.2"
type: Class
title: ActiveBarItem
description: "One slot in the Active Bar — either a clickable button, a thin"
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/ActiveBarItem
language: rust
---

# ActiveBarItem

One slot in the Active Bar — either a clickable button, a thin

## Signature

```rust
pub enum ActiveBarItem
```

## Type Parameters

- `M: 'static + Clone`

## Visibility

- `pub`

## Docstring

One slot in the Active Bar — either a clickable button, a thin
vertical separator, or an arbitrary user-supplied widget (escape
hatch for special cases like the schematic's draw-mode pill).

## Methods

- `content`
- `width`

## Source
Lines 56–72 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
