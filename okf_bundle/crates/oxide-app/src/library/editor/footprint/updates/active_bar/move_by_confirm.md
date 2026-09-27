---
okf_version: "0.2"
type: Function
title: move_by_confirm
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_confirm
language: rust
---

# move_by_confirm

## Signature

```rust
fn move_by_confirm(editor: &mut crate::app::FootprintEditorState)
```

## Source
Lines 255–260 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [footprint_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/mod/footprint_nudge_selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
