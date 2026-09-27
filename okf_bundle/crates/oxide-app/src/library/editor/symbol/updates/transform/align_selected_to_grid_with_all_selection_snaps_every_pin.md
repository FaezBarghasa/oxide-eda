---
okf_version: "0.2"
type: Function
title: align_selected_to_grid_with_all_selection_snaps_every_pin
description: "#426 — `All` is alignable (unlike Delete, snapping never"
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_all_selection_snaps_every_pin
language: rust
---

# align_selected_to_grid_with_all_selection_snaps_every_pin

#426 — `All` is alignable (unlike Delete, snapping never

## Signature

```rust
fn align_selected_to_grid_with_all_selection_snaps_every_pin()
```

## Decorators

- `test`

## Docstring

#426 — `All` is alignable (unlike Delete, snapping never
destroys data), so a full-symbol Align To Grid actually snaps
every pin and graphic rather than silently no-op'ing.
[test]

## Source
Lines 230–243 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
