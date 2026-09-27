---
okf_version: "0.2"
type: Function
title: handle_focus_at
description: Move the viewport to center on a world-space point and optionally
resource: crates/oxide-app/src/app/handlers/erc/erc_run.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/erc/erc_run/handle_focus_at_1
language: rust
---

# handle_focus_at

Move the viewport to center on a world-space point and optionally

## Signature

```rust
pub(crate) fn handle_focus_at(
        &mut self,
        world_x: f64,
        world_y: f64,
        select: Option<oxide_types::schematic::SelectedItem>,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Move the viewport to center on a world-space point and optionally
replace the current selection. Used by the ERC panel's
click-to-zoom and by future Find/Replace result navigation.

## Source
Lines 406–432 in `crates/oxide-app/src/app/handlers/erc/erc_run.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc_run](/crates/oxide-app/src/app/handlers/erc/erc_run.md) |
