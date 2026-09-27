---
okf_version: "0.2"
type: Function
title: collect_exposed_sheet_ports
description: Returns the ports this sheet exposes to a parent hierarchical symbol.
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/collect_exposed_sheet_ports_1
language: rust
---

# collect_exposed_sheet_ports

Returns the ports this sheet exposes to a parent hierarchical symbol.

## Signature

```rust
pub fn collect_exposed_sheet_ports(&self) -> Vec<SheetPort>
```

## Visibility

- `pub`

## Docstring

Returns the ports this sheet exposes to a parent hierarchical symbol.
Hierarchical labels have precedence over global labels with the same name.

## Source
Lines 10–38 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| calls | [port_label_priority](/crates/oxide-engine/src/sheet/port_label_priority.md) |
| calls | [normalize_sheet_pin_direction](/crates/oxide-engine/src/sheet/normalize_sheet_pin_direction.md) |
