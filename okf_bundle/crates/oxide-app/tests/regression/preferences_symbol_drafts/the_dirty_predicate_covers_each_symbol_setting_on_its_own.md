---
okf_version: "0.2"
type: Function
title: the_dirty_predicate_covers_each_symbol_setting_on_its_own
description: "`UiState::preferences_draft_differs` is the single predicate behind the"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/the_dirty_predicate_covers_each_symbol_setting_on_its_own
language: rust
---

# the_dirty_predicate_covers_each_symbol_setting_on_its_own

`UiState::preferences_draft_differs` is the single predicate behind the

## Signature

```rust
fn the_dirty_predicate_covers_each_symbol_setting_on_its_own()
```

## Decorators

- `test`

## Docstring

`UiState::preferences_draft_differs` is the single predicate behind the
footer and every dirty-close guard, and its doc comment claims to cover
ALL draft state. Each of the three has to be in it individually — a
predicate that happens to catch one of them would let the other two be
lost on close.
[test]

## Source
Lines 133–167 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| calls | [assert_dirty_on](/crates/oxide-app/tests/regression/preferences_symbol_drafts/assert_dirty_on.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_grid_style.md) |
| calls | [other_pin_selection](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_pin_selection.md) |
