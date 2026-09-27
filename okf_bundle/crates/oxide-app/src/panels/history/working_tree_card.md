---
okf_version: "0.2"
type: Function
title: working_tree_card
description: "\"Working tree (uncommitted changes)\" pseudo-card pinned above the"
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/working_tree_card
language: rust
---

# working_tree_card

"Working tree (uncommitted changes)" pseudo-card pinned above the

## Signature

```rust
fn working_tree_card(
    primary: iced::Color,
    muted: iced::Color,
    border_c: iced::Color,
    bg: Option<iced::Background>,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Docstring

"Working tree (uncommitted changes)" pseudo-card pinned above the
committed history when the active file is dirty. Same visual
shape as a `oxide_widgets::history_pane` card so it reads as a
peer of the actual commits.

## Source
Lines 283–308 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
