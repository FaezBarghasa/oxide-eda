---
okf_version: "0.2"
type: Module
title: preferences_symbol_drafts
description: "#629 — the three Symbol Editor appearance settings must behave like"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts
language: rust
---

# preferences_symbol_drafts

#629 — the three Symbol Editor appearance settings must behave like

## Docstring

#629 — the three Symbol Editor appearance settings must behave like
drafts, because that is what the pane they live in promises.

They used to write `prefs.json` the moment the picker moved, while
every other setting under the same Save/Cancel footer waited for Save.
Worse, they had no committed `UiState` field at all — the
`preferences_draft_*` field WAS the saved value, seeded from disk at
boot — so `revert_preferences_drafts` had nothing to restore from and
Cancel silently kept the change.

**No test here touches the filesystem.** `PrefMsg::Save` writes the
per-process prefs path that `preferences_prefs_recovery.rs` guards with
its own module `Mutex`, and the regression tests are one binary with no
shared lock between modules (see `tests/regression.rs`), so driving
Save from here would race that guard. The Save arm's three new commit
lines are therefore NOT covered by a test — they sit directly beneath
the four identical lines for `power_port_style`, `label_style`,
`multisheet_style` and `grid_style`. Closing that gap needs a lock
shared across modules first.

## Relationships

| Type | Target |
|------|--------|
| related | [inner](/crates/oxide-app/tests/regression/preferences_symbol_drafts/inner.md) |
| related | [other_grid_style](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_grid_style.md) |
| related | [other_pin_selection](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_pin_selection.md) |
| related | [changing_a_symbol_setting_marks_the_dialog_dirty_without_committing](/crates/oxide-app/tests/regression/preferences_symbol_drafts/changing_a_symbol_setting_marks_the_dialog_dirty_without_committing.md) |
| related | [discarding_puts_all_three_symbol_drafts_back](/crates/oxide-app/tests/regression/preferences_symbol_drafts/discarding_puts_all_three_symbol_drafts_back.md) |
| related | [the_dirty_predicate_covers_each_symbol_setting_on_its_own](/crates/oxide-app/tests/regression/preferences_symbol_drafts/the_dirty_predicate_covers_each_symbol_setting_on_its_own.md) |
| related | [assert_dirty_on](/crates/oxide-app/tests/regression/preferences_symbol_drafts/assert_dirty_on.md) |
