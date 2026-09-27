---
okf_version: "0.2"
type: Class
title: PrefsView
description: Everything the Preferences dialog reads to render one frame.
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/PrefsView
language: rust
---

# PrefsView

Everything the Preferences dialog reads to render one frame.

## Signature

```rust
pub struct PrefsView
```

## Type Parameters

- `'a`

## Decorators

- `derive(Clone, Copy)`

## Visibility

- `pub`

## Docstring

Everything the Preferences dialog reads to render one frame.

The dialog is a four-hop chain — `view` / `view_body` -> `build_dialog`
-> `build_content` — and every hop needs very nearly the same inputs.
Passed positionally that was 23 parameters per hop, and several share a
type: `draft_theme`, `saved_theme` and `theme_id` are all `ThemeId`;
`draft_grid_style` and `draft_symbol_grid_style` are both `GridStyle`.
A transposed pair would have compiled silently and shipped the wrong
control state. Naming the fields makes that a compile error instead.

Every field is `Copy`, so the whole struct is passed by value and each
hop costs a move rather than a fresh borrow.
[derive(Clone, Copy)]

## Methods

- `nav`
- `draft_theme`
- `saved_theme`
- `draft_font`
- `draft_power_port_style`
- `draft_label_style`
- `draft_multisheet_style`
- `draft_grid_style`
- `draft_pcb_gpu_render`
- `draft_symbol_grid_size_mm`
- `draft_symbol_grid_style`
- `draft_symbol_pin_selection`
- `custom_name`
- `theme_status`
- `dirty`
- `erc_overrides`
- `distributor_settings`
- `panel_tokens`
- `draft_component_classes`
- `keymap_editor`
- `keymap_status`
- `keymap_load_error`
- `keymap_backup`
- `prefs_load_error`
- `prefs_status`
- `keymap_search`
- `keymap_recorder`
- `theme_id`

## Source
Lines 265–313 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
