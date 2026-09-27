---
okf_version: "0.2"
type: Class
title: SymbolEditorPanelContext
description: Context handed to the right-dock Properties panel and the SCH-Library
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/SymbolEditorPanelContext
language: rust
---

# SymbolEditorPanelContext

Context handed to the right-dock Properties panel and the SCH-Library

## Signature

```rust
pub struct SymbolEditorPanelContext
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Context handed to the right-dock Properties panel and the SCH-Library
left-dock panel when the active tab is a `.snxsym` standalone editor.
Mirrors the live `SymbolEditorState` but contains only the fields the
panels render — keeps `panel_ctx` cloneable and decouples panel code
from the canvas state struct.
[derive(Debug, Clone)]

## Methods

- `path`
- `symbol_name`
- `symbol_designator`
- `symbol_comment`
- `symbol_description`
- `symbol_component_type`
- `symbol_mirrored`
- `symbol_local_fill_color`
- `symbol_local_line_color`
- `symbol_local_pin_color`
- `symbol_uuid`
- `pins`
- `graphics`
- `selected`
- `graphic_fill_picker`
- `local_color_picker`
- `symbols_in_file`
- `active_idx`
- `active_part`
- `active_max_part`
- `active_has_part_zero`
- `display`

## Source
Lines 11–76 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
