---
okf_version: "0.2"
type: Function
title: read_dock_layout
description: Rebuild a DockArea from persisted JSON. Returns None when no saved
resource: crates/oxide-app/src/fonts/dock_layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:28Z"
concept_id: crates/oxide-app/src/fonts/dock_layout/read_dock_layout
language: rust
---

# read_dock_layout

Rebuild a DockArea from persisted JSON. Returns None when no saved

## Signature

```rust
pub fn read_dock_layout() -> Option<crate::dock::DockArea>
```

## Visibility

- `pub`

## Docstring

Rebuild a DockArea from persisted JSON. Returns None when no saved
layout exists so the caller can fall back to the default seed.

## Source
Lines 37–69 in `crates/oxide-app/src/fonts/dock_layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dock_layout](/crates/oxide-app/src/fonts/dock_layout.md) |
| calls | [parse_panel_kind](/crates/oxide-app/src/fonts/dock_layout/parse_panel_kind.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
