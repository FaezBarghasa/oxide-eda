---
okf_version: "0.2"
type: Function
title: dropdown_trigger_items
description: "Build the 8 dropdown trigger buttons matching the schematic's"
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_trigger_items
language: rust
---

# dropdown_trigger_items

Build the 8 dropdown trigger buttons matching the schematic's

## Signature

```rust
fn dropdown_trigger_items(
    editor: &FootprintEditorState,
    tid: ThemeId,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Docstring

Build the 8 dropdown trigger buttons matching the schematic's
pattern: left-click fires the default action (or toggles the
menu when there's no obvious default — Filter / Snap), right-click
opens the dropdown. Chevron indicator advertises the right-click
secondary action.

## Source
Lines 178–273 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| called_by | [bar_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items.md) |
