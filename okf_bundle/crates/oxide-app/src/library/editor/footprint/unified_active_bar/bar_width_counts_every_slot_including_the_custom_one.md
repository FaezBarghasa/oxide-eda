---
okf_version: "0.2"
type: Function
title: bar_width_counts_every_slot_including_the_custom_one
description: "The measured width has to match what the bar actually draws, or"
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_width_counts_every_slot_including_the_custom_one
language: rust
---

# bar_width_counts_every_slot_including_the_custom_one

The measured width has to match what the bar actually draws, or

## Signature

```rust
fn bar_width_counts_every_slot_including_the_custom_one()
```

## Decorators

- `test`

## Docstring

The measured width has to match what the bar actually draws, or
the centre-aligned bar's left edge is wrong and every panel
shifts by half the error. Guards the Custom slot in particular:
its width is declared, not measured.
[test]

## Source
Lines 343–375 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [editor_in](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/editor_in.md) |
| calls | [bar_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items.md) |
| calls | [menu_trigger_geometry](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry.md) |
