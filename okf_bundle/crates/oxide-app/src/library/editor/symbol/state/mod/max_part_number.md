---
okf_version: "0.2"
type: Function
title: max_part_number
description: "Highest declared part number across every pin on `sym`, reconciled"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number
language: rust
---

# max_part_number

Highest declared part number across every pin on `sym`, reconciled

## Signature

```rust
pub fn max_part_number(sym: &Symbol) -> u8
```

## Visibility

- `pub`

## Docstring

Highest declared part number across every pin on `sym`, reconciled
with the first-class `Symbol.part_count`. `0` (Part Zero) is
excluded — it's the special "appears on every part" marker, not a
real part. Maxing against `part_count` (rather than reading pins
alone) is what lets an empty New Part (no pins yet) survive
navigate + save. Returns `1` for symbols with no pins or only Part
Zero pins so multi-part wiring still has a sensible "current max
part = 1" baseline.

## Source
Lines 279–291 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [sym_editor_select_part](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_part.md) |
| called_by | [build_symbol_editor_panel_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx/build_symbol_editor_panel_ctx.md) |
| called_by | [view_symbol_toolbar](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar.md) |
| called_by | [delete_unit](/crates/oxide-app/src/library/editor/symbol/state/mod/delete_unit.md) |
| called_by | [clamp_active_part](/crates/oxide-app/src/library/editor/symbol/updates/history/clamp_active_part.md) |
| called_by | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
