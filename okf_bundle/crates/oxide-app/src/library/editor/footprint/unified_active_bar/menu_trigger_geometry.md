---
okf_version: "0.2"
type: Function
title: menu_trigger_geometry
description: "Where `menu`'s trigger button sits: its left edge in px from the"
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry
language: rust
---

# menu_trigger_geometry

Where `menu`'s trigger button sits: its left edge in px from the

## Signature

```rust
fn menu_trigger_geometry(
    items: &[ActiveBarItem<LibraryMessage>],
    menu: FpActiveBarMenu,
) -> (Option<f32>, f32)
```

## Docstring

Where `menu`'s trigger button sits: its left edge in px from the
bar's own left edge, plus the bar's total width. `None` when the
current bar has no trigger for that menu (the two sketch group
menus only exist in `EditorMode::Sketch`).

Both numbers are **derived from the items the bar just built**, not
from a table of slot indices. That matters: the previous version
hand-maintained a `Filter => 0, Snap => 1, …` index map plus its own
copies of the widget's pixel constants, so every button added,
removed, or reordered was a chance to silently misplace every panel.

The trigger is located by its *message*, not its position — every
menu trigger publishes `ToggleActiveBarMenu(menu)` on right-press,
while the left-press action varies (Place / Select / Text arm a tool
instead). Nothing else on the bar sends that message.

## Source
Lines 63–81 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| calls | [slot_offsets](/crates/oxide-widgets/src/active_bar/mod/slot_offsets.md) |
| called_by | [bar_width_counts_every_slot_including_the_custom_one](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_width_counts_every_slot_including_the_custom_one.md) |
| called_by | [dropdown_overlay](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_overlay.md) |
| called_by | [menu_triggers_are_located_by_message_not_by_index](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_triggers_are_located_by_message_not_by_index.md) |
