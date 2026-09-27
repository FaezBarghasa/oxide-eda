---
okf_version: "0.2"
type: Function
title: escape_for_standard
resource: crates/oxide-app/src/schematic_runtime/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/text/escape_for_standard
language: rust
---

# escape_for_standard

## Signature

```rust
pub fn escape_for_standard(text: &str) -> String
```

## Visibility

- `pub`

## Source
Lines 7–9 in `crates/oxide-app/src/schematic_runtime/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-app/src/schematic_runtime/text.md) |
| called_by | [dispatch_text_edit_message](/crates/oxide-app/src/app/dispatch/text_edit/dispatch_text_edit_message.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [handle_dock_property_editor_message](/crates/oxide-app/src/app/handlers/dock/property_editor/handle_dock_property_editor_message.md) |
