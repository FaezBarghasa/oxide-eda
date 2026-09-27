---
okf_version: "0.2"
type: Function
title: action_enabled
description: "Whether `action` is clickable given the current selection and"
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/action_enabled
language: rust
---

# action_enabled

Whether `action` is clickable given the current selection and

## Signature

```rust
pub(super) fn action_enabled(
    action: &ActiveBarAction,
    has_selection: bool,
    has_net_colors: bool,
) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Whether `action` is clickable given the current selection and
net-colour state.

#633 — these two facts used to reach here through a pair of
thread-locals that `view_bar` set for the duration of one render.
Their reset value was `true`, so any path reaching a bar helper
outside `view_bar` rendered a fully-enabled bar regardless of what
was selected — a fallback that silently produced the wrong UI
instead of failing. They are arguments now, so the compiler asks.

## Source
Lines 68–80 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
| calls | [requires_selection](/crates/oxide-app/src/active_bar/mod/requires_selection.md) |
| calls | [requires_net_color](/crates/oxide-app/src/active_bar/mod/requires_net_color.md) |
| called_by | [dd_item](/crates/oxide-app/src/active_bar/dropdown/dd_item.md) |
| called_by | [view_bar](/crates/oxide-app/src/active_bar/mod/view_bar.md) |
