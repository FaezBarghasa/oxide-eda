---
okf_version: "0.2"
type: Function
title: panel_kinds
description: "Panels currently docked in `position`, in display order. Used"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/panel_kinds_1
language: rust
---

# panel_kinds

Panels currently docked in `position`, in display order. Used

## Signature

```rust
pub fn panel_kinds(&self, position: PanelPosition) -> &[panels::PanelKind]
```

## Visibility

- `pub`

## Docstring

Panels currently docked in `position`, in display order. Used
by the Panels menu to mark open panels with a ✓.

## Source
Lines 321–327 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
