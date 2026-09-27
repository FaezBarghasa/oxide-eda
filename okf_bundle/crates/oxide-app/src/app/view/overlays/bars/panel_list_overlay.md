---
okf_version: "0.2"
type: Function
title: panel_list_overlay
description: "v0.18.10 status-bar panel list popup. Anchored above the \"Panels\""
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/panel_list_overlay
language: rust
---

# panel_list_overlay

v0.18.10 status-bar panel list popup. Anchored above the "Panels"

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn panel_list_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.18.10 status-bar panel list popup. Anchored above the "Panels"
button in the bottom-right of the status bar; each row shows a ✓
when the panel is open somewhere (docked, floating, or detached).
Pushes the dismiss layer then the popup.

## Source
Lines 645–736 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| calls | [context_menu](/crates/oxide-app/src/styles/context_menu.md) |
