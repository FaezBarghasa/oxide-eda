---
okf_version: "0.2"
type: Function
title: inner
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/inner
language: rust
---

# inner

## Signature

```rust
fn inner(msg: PrefMsg) -> Message
```

## Source
Lines 25–27 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| called_by | [changing_a_symbol_setting_marks_the_dialog_dirty_without_committing](/crates/oxide-app/tests/regression/preferences_symbol_drafts/changing_a_symbol_setting_marks_the_dialog_dirty_without_committing.md) |
| called_by | [discarding_puts_all_three_symbol_drafts_back](/crates/oxide-app/tests/regression/preferences_symbol_drafts/discarding_puts_all_three_symbol_drafts_back.md) |
