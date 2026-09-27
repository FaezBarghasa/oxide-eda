---
okf_version: "0.2"
type: Function
title: apply_symbol_ui
resource: crates/oxide-app/src/library/editor/symbol/updates/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui
language: rust
---

# apply_symbol_ui

## Signature

```rust
pub(super) fn apply_symbol_ui(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–59 in `crates/oxide-app/src/library/editor/symbol/updates/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/symbol/updates/ui.md) |
| calls | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [set_tool_away_from_polygon_commits_synchronously](/crates/oxide-app/src/library/editor/symbol/updates/ui/set_tool_away_from_polygon_commits_synchronously.md) |
| called_by | [set_tool_away_from_polygon_discards_short_stash](/crates/oxide-app/src/library/editor/symbol/updates/ui/set_tool_away_from_polygon_discards_short_stash.md) |
