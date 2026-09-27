---
okf_version: "0.2"
type: Function
title: drawing_num_row
description: Buffer-backed numeric row — survives empty / partial input so the
resource: crates/oxide-app/src/panels/element_properties/drawing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing/drawing_num_row
language: rust
---

# drawing_num_row

Buffer-backed numeric row — survives empty / partial input so the

## Signature

```rust
fn drawing_num_row(
    label: &'a str,
    field: DrawingFieldId,
    stored_value: f64,
    buf: &std::collections::HashMap<DrawingFieldId, String>,
    muted: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Buffer-backed numeric row — survives empty / partial input so the
user can erase and retype the whole value. Emits DrawingFieldTyping
on every keystroke; the handler commits to the engine when the
string parses as f64.

## Source
Lines 388–414 in `crates/oxide-app/src/panels/element_properties/drawing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing](/crates/oxide-app/src/panels/element_properties/drawing.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
