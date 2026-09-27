---
okf_version: "0.2"
type: Function
title: sym_editor_select_graphic
description: "SCH Library panel: select a placed graphic so the right-dock"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_graphic_1
language: rust
---

# sym_editor_select_graphic

SCH Library panel: select a placed graphic so the right-dock

## Signature

```rust
pub(super) fn sym_editor_select_graphic(&mut self, idx: usize) -> bool
```

## Visibility

- `pub(super)`

## Docstring

SCH Library panel: select a placed graphic so the right-dock
Properties panel renders its per-shape fields. Mirrors
[`sym_editor_select_pin`].

## Source
Lines 268–280 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
