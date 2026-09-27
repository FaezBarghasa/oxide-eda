---
okf_version: "0.2"
type: Function
title: mutates_footprint_state
description: v0.24 Phase 1 (Track B) — message-kind classifier driving the
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/mutates_footprint_state
language: rust
---

# mutates_footprint_state

v0.24 Phase 1 (Track B) — message-kind classifier driving the

## Signature

```rust
fn mutates_footprint_state(msg: &FootprintEditorMsg) -> bool
```

## Docstring

v0.24 Phase 1 (Track B) — message-kind classifier driving the
`push_history` decision in [`apply_footprint_primitive_edit`].
Returns `true` for messages that mutate persisted footprint /
sketch state (so undo can roll them back), `false` for pure UI
state (selection, cursor tracking, tool mode toggles, panel
pickers — these don't enter the history because rolling back a
"click happened here" doesn't make sense to the user).

Lean toward `true` when in doubt — extra history entries cost
memory but never break correctness; missing entries leave edits
unreversable.

## Source
Lines 491–606 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| called_by | [apply_footprint_primitive_edit](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit.md) |
