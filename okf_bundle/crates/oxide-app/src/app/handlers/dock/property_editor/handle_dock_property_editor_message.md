---
okf_version: "0.2"
type: Function
title: handle_dock_property_editor_message
resource: crates/oxide-app/src/app/handlers/dock/property_editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/property_editor/handle_dock_property_editor_message
language: rust
---

# handle_dock_property_editor_message

## Signature

```rust
impl Oxide { pub(super) fn handle_dock_property_editor_message(
        &mut self,
        panel_msg: &crate::panels::PanelMsg,
    ) -> bool }
```

## Visibility

- `pub(super)`

## Source
Lines 4–322 in `crates/oxide-app/src/app/handlers/dock/property_editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [property_editor](/crates/oxide-app/src/app/handlers/dock/property_editor.md) |
| calls | [escape_for_standard](/crates/oxide-app/src/schematic_runtime/text/escape_for_standard.md) |
| calls | [Label](/crates/oxide-types/src/schematic/sheet/Label.md) |
| calls | [TextNote](/crates/oxide-types/src/schematic/sheet/TextNote.md) |
| calls | [iced_color_to_stroke](/crates/oxide-app/src/app/handlers/dock/property_editor/iced_color_to_stroke.md) |
