---
okf_version: "0.2"
type: Function
title: symbol_value_position
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/symbol_value_position
language: rust
---

# symbol_value_position

## Signature

```rust
impl SchematicSheet { fn symbol_value_position(&self, uuid: uuid::Uuid) -> Option<(f64, f64)> }
```

## Source
Lines 111–117 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
