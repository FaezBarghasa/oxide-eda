---
okf_version: "0.2"
type: Function
title: changing_a_symbol_setting_marks_the_dialog_dirty_without_committing
description: "A draft is a draft: moving the picker must leave the committed value"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/changing_a_symbol_setting_marks_the_dialog_dirty_without_committing
language: rust
---

# changing_a_symbol_setting_marks_the_dialog_dirty_without_committing

A draft is a draft: moving the picker must leave the committed value

## Signature

```rust
fn changing_a_symbol_setting_marks_the_dialog_dirty_without_committing()
```

## Decorators

- `test`

## Docstring

A draft is a draft: moving the picker must leave the committed value
alone and light up the Save/Discard footer. Before the fix the handler
wrote straight to `prefs.json` and never called
`recompute_preferences_dirty`, so the footer stayed dark while the
change was already permanent.
[test]

## Source
Lines 52–75 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_grid_style.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_symbol_drafts/inner.md) |
