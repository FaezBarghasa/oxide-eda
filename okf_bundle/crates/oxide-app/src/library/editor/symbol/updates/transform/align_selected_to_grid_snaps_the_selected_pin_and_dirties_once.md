---
okf_version: "0.2"
type: Function
title: align_selected_to_grid_snaps_the_selected_pin_and_dirties_once
description: "#426 — a real selection snaps onto the 1.27 mm symbol-canvas"
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_snaps_the_selected_pin_and_dirties_once
language: rust
---

# align_selected_to_grid_snaps_the_selected_pin_and_dirties_once

#426 — a real selection snaps onto the 1.27 mm symbol-canvas

## Signature

```rust
fn align_selected_to_grid_snaps_the_selected_pin_and_dirties_once()
```

## Decorators

- `test`

## Docstring

#426 — a real selection snaps onto the 1.27 mm symbol-canvas
grid and dirties/snapshots exactly once (dispatch test, driving
the message through the same `apply_symbol_transform` entry
point the active bar uses).
[test]

## Source
Lines 165–190 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
