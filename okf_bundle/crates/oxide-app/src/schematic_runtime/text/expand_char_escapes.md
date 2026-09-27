---
okf_version: "0.2"
type: Function
title: expand_char_escapes
resource: crates/oxide-app/src/schematic_runtime/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/text/expand_char_escapes
language: rust
---

# expand_char_escapes

## Signature

```rust
pub fn expand_char_escapes(text: &str) -> String
```

## Visibility

- `pub`

## Source
Lines 3–5 in `crates/oxide-app/src/schematic_runtime/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-app/src/schematic_runtime/text.md) |
| called_by | [handle_canvas_double_clicked](/crates/oxide-app/src/app/handlers/canvas/double_clicked/handle_canvas_double_clicked.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
