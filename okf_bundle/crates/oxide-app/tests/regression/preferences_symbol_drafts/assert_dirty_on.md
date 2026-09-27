---
okf_version: "0.2"
type: Function
title: assert_dirty_on
description: "Open a fresh dialog, move exactly one draft, and require the"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/assert_dirty_on
language: rust
---

# assert_dirty_on

Open a fresh dialog, move exactly one draft, and require the

## Signature

```rust
fn assert_dirty_on(label: &str, mutate: impl FnOnce(&mut Oxide)
```

## Docstring

Open a fresh dialog, move exactly one draft, and require the
predicate to notice. One setting per call so a predicate that
happens to catch a different term cannot carry this one.

## Source
Lines 137–154 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| called_by | [the_dirty_predicate_covers_each_symbol_setting_on_its_own](/crates/oxide-app/tests/regression/preferences_symbol_drafts/the_dirty_predicate_covers_each_symbol_setting_on_its_own.md) |
