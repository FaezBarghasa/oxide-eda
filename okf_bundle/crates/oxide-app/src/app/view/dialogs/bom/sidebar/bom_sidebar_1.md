---
okf_version: "0.2"
type: Function
title: bom_sidebar
description: Build the Properties sidebar column (tab strip + active pane) for the
resource: crates/oxide-app/src/app/view/dialogs/bom/sidebar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/sidebar/bom_sidebar_1
language: rust
---

# bom_sidebar

Build the Properties sidebar column (tab strip + active pane) for the

## Signature

```rust
pub(super) fn bom_sidebar(&self) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

Build the Properties sidebar column (tab strip + active pane) for the
active preview. Returns the sidebar `column` the modal drops into its
main row.

## Source
Lines 17–332 in `crates/oxide-app/src/app/view/dialogs/bom/sidebar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sidebar](/crates/oxide-app/src/app/view/dialogs/bom/sidebar.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
