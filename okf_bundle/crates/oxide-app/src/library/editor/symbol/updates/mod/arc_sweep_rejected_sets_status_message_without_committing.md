---
okf_version: "0.2"
type: Function
title: arc_sweep_rejected_sets_status_message_without_committing
description: "`ArcSweepRejected` surfaces a status message and commits"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/arc_sweep_rejected_sets_status_message_without_committing
language: rust
---

# arc_sweep_rejected_sets_status_message_without_committing

`ArcSweepRejected` surfaces a status message and commits

## Signature

```rust
fn arc_sweep_rejected_sets_status_message_without_committing()
```

## Decorators

- `test`

## Docstring

`ArcSweepRejected` surfaces a status message and commits
nothing — no graphic, no undo snapshot. The gesture-level
"third click ignored" behavior lives in `canvas::input::tools`
(see `arc_sweep_exceeds_full_turn`'s tests); this covers the
message-dispatch half of the fix.
[test]

## Source
Lines 885–895 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
