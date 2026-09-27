---
okf_version: "0.2"
type: Function
title: menu_triggers_are_located_by_message_not_by_index
description: Panel anchors are located by scanning the built bar for the
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_triggers_are_located_by_message_not_by_index
language: rust
---

# menu_triggers_are_located_by_message_not_by_index

Panel anchors are located by scanning the built bar for the

## Signature

```rust
fn menu_triggers_are_located_by_message_not_by_index()
```

## Decorators

- `test`

## Docstring

Panel anchors are located by scanning the built bar for the
button that opens each menu. Confirm the scan actually finds
them, in the right order, and that a menu whose trigger isn't on
the current bar reports `None` instead of a bogus offset.
[test]

## Source
Lines 296–336 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [editor_in](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/editor_in.md) |
| calls | [bar_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items.md) |
| calls | [menu_trigger_geometry](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry.md) |
