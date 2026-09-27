---
okf_version: "0.2"
type: Class
title: Label
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/Label
language: rust
---

# Label

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Label
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `uuid`
- `text`
- `position`
- `rotation`
- `label_type`
- `shape`
- `font_size`
- `justify`
- `justify_v`

## Source
Lines 37–52 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
| called_by | [dispatch_text_edit_message](/crates/oxide-app/src/app/dispatch/text_edit/dispatch_text_edit_message.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [handle_dock_property_editor_message](/crates/oxide-app/src/app/handlers/dock/property_editor/handle_dock_property_editor_message.md) |
| called_by | [refresh_find_matches](/crates/oxide-app/src/app/handlers/find_replace/refresh_find_matches.md) |
