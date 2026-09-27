---
okf_version: "0.2"
type: Function
title: bar_items
description: Build the bar items only — caller mounts via
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items
language: rust
---

# bar_items

Build the bar items only — caller mounts via

## Signature

```rust
pub fn bar_items(
    editor: &FootprintEditorState,
    theme_id: ThemeId,
    tokens: &ThemeTokens,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Visibility

- `pub`

## Docstring

Build the bar items only — caller mounts via
`oxide_widgets::active_bar::view(items, tokens)` so the chain is
identical to the schematic.

## Source
Lines 23–46 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| calls | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_trigger_items.md) |
| called_by | [bar_width_counts_every_slot_including_the_custom_one](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_width_counts_every_slot_including_the_custom_one.md) |
| called_by | [dropdown_overlay](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_overlay.md) |
| called_by | [menu_triggers_are_located_by_message_not_by_index](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_triggers_are_located_by_message_not_by_index.md) |
