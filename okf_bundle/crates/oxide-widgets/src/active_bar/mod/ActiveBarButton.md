---
okf_version: "0.2"
type: Class
title: ActiveBarButton
description: One clickable button in the Active Bar.
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/ActiveBarButton
language: rust
---

# ActiveBarButton

One clickable button in the Active Bar.

## Signature

```rust
pub struct ActiveBarButton
```

## Type Parameters

- `M: 'static + Clone`

## Visibility

- `pub`

## Docstring

One clickable button in the Active Bar.

`M` is the editor's message type; the bar is generic over it so
every editor can publish its own messages without sharing a
common message enum.

## Methods

- `icon`
- `tooltip`
- `enabled`
- `selected`
- `on_press`
- `on_right_press`
- `dropdown_indicator`

## Source
Lines 123–149 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
