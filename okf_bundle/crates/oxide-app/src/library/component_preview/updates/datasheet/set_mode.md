---
okf_version: "0.2"
type: Function
title: set_mode
description: Switch the datasheet reference between URL and pinned-PDF modes.
resource: crates/oxide-app/src/library/component_preview/updates/datasheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:52Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/datasheet/set_mode
language: rust
---

# set_mode

Switch the datasheet reference between URL and pinned-PDF modes.

## Signature

```rust
pub(super) fn set_mode(state: &mut ComponentPreviewState, mode: DatasheetMode)
```

## Visibility

- `pub(super)`

## Docstring

Switch the datasheet reference between URL and pinned-PDF modes.

Resets the reference to a fresh default of the chosen kind only when
the current value is of the other kind, so re-selecting the active
mode is a no-op and does not clobber the existing value.

## Source
Lines 15–35 in `crates/oxide-app/src/library/component_preview/updates/datasheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet](/crates/oxide-app/src/library/component_preview/updates/datasheet.md) |
