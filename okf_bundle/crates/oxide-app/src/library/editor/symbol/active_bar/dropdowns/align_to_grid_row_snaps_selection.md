---
okf_version: "0.2"
type: Function
title: align_to_grid_row_snaps_selection
description: "#426 — the Align dropdown's \"Align To Grid\" row used to be a"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_to_grid_row_snaps_selection
language: rust
---

# align_to_grid_row_snaps_selection

#426 — the Align dropdown's "Align To Grid" row used to be a

## Signature

```rust
fn align_to_grid_row_snaps_selection()
```

## Decorators

- `test`

## Docstring

#426 — the Align dropdown's "Align To Grid" row used to be a
dead `ActiveBarStub`. It now dispatches the real
`AlignSelectedToGrid` snap.
[test]

## Source
Lines 459–465 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| calls | [align_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/align_entries.md) |
