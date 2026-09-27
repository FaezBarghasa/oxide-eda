---
okf_version: "0.2"
type: Function
title: other_grid_style
description: "Pick the variant the app is not currently showing, so the test asserts"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/other_grid_style
language: rust
---

# other_grid_style

Pick the variant the app is not currently showing, so the test asserts

## Signature

```rust
fn other_grid_style(current: GridStyle) -> GridStyle
```

## Docstring

Pick the variant the app is not currently showing, so the test asserts
on a real change rather than a no-op assignment that would pass against
a handler that dropped the message entirely.

## Source
Lines 32–37 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| called_by | [changing_a_symbol_setting_marks_the_dialog_dirty_without_committing](/crates/oxide-app/tests/regression/preferences_symbol_drafts/changing_a_symbol_setting_marks_the_dialog_dirty_without_committing.md) |
| called_by | [discarding_puts_all_three_symbol_drafts_back](/crates/oxide-app/tests/regression/preferences_symbol_drafts/discarding_puts_all_three_symbol_drafts_back.md) |
| called_by | [the_dirty_predicate_covers_each_symbol_setting_on_its_own](/crates/oxide-app/tests/regression/preferences_symbol_drafts/the_dirty_predicate_covers_each_symbol_setting_on_its_own.md) |
