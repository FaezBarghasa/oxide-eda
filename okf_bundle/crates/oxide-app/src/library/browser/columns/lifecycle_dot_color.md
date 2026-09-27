---
okf_version: "0.2"
type: Function
title: lifecycle_dot_color
description: "Per-lifecycle indicator dot colour. Matches plan §6:"
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns/lifecycle_dot_color
language: rust
---

# lifecycle_dot_color

Per-lifecycle indicator dot colour. Matches plan §6:

## Signature

```rust
pub(super) fn lifecycle_dot_color(state: LifecycleState) -> iced::Color
```

## Visibility

- `pub(super)`

## Docstring

Per-lifecycle indicator dot colour. Matches plan §6:

* Released → green;
* Draft / InReview → neutral grey ("active, but not preferred");
* Deprecated → amber/yellow;
* Obsolete → muted dark grey.

Centralised here so both the dot and any future lifecycle badge
in the side preview pane can pull the same colour.

## Source
Lines 277–288 in `crates/oxide-app/src/library/browser/columns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [columns](/crates/oxide-app/src/library/browser/columns.md) |
| called_by | [view_grid](/crates/oxide-app/src/library/browser/grid/view_grid.md) |
