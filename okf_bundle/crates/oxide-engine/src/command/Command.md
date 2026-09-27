---
okf_version: "0.2"
type: Class
title: Command
description: "[derive(Debug, Clone)]"
resource: crates/oxide-engine/src/command.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-engine/src/command/Command
language: rust
---

# Command

[derive(Debug, Clone)]

## Signature

```rust
pub enum Command
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone)]

## Methods

- `document`
- `items`
- `dx`
- `dy`
- `items`
- `angle_degrees`
- `items`
- `axis`
- `items`
- `target`
- `value`
- `label_id`
- `font_size_mm`
- `justify`
- `rotation_degrees`
- `symbol_id`
- `rotation_degrees`
- `symbol_id`
- `field`
- `font_size_mm`
- `symbol_id`
- `footprint`
- `symbol_id`
- `lib_id`
- `symbol_id`
- `reference`
- `value`
- `footprint`
- `symbol_id`
- `key`
- `value`
- `wire`
- `bus`
- `label`
- `symbol`
- `junction`
- `no_connect`
- `bus_entry`
- `text_note`
- `drawing`
- `drawing`
- `sheet_id`
- `stroke_width`
- `stroke_color`
- `fill_color`
- `mode`
- `symbol_id`
- `x`
- `y`
- `items`
- `direction`
- `child_filename`
- `ports`
- `paper_size`

## Source
Lines 66–218 in `crates/oxide-engine/src/command.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-engine/src/command.md) |
| called_by | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| called_by | [apply_active_trigger](/crates/oxide-app/src/keymap/editor/apply_active_trigger.md) |
| called_by | [into_bindings](/crates/oxide-app/src/keymap/profile/into_bindings.md) |
