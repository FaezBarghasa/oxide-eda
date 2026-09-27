---
okf_version: "0.2"
type: Function
title: theme_import_dirty_flag_survives_an_unrelated_appearance_toggle
description: "Finding 1 (data-loss): `preferences_draft_differs()` used to compare only"
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard/theme_import_dirty_flag_survives_an_unrelated_appearance_toggle
language: rust
---

# theme_import_dirty_flag_survives_an_unrelated_appearance_toggle

Finding 1 (data-loss): `preferences_draft_differs()` used to compare only

## Signature

```rust
fn theme_import_dirty_flag_survives_an_unrelated_appearance_toggle()
```

## Decorators

- `test`

## Docstring

Finding 1 (data-loss): `preferences_draft_differs()` used to compare only
the 7 appearance drafts, so an imported theme (which sets
`preferences_dirty` imperatively via `custom_theme`, invisible to that
comparator) got silently clobbered back to "clean" by the next unrelated
appearance-draft recompute — exactly reproducing the review's repro:
theme ALREADY Custom (so `preferences_draft_theme` reads `Custom` both
before and after the import — the enum-tag comparison alone can't see a
same-tag content swap) → Import Theme replaces `custom_theme`'s content →
toggle an unrelated appearance draft.
[test]

## Source
Lines 31–91 in `crates/oxide-app/tests/regression/preferences_dirty_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_dirty_guard](/crates/oxide-app/tests/regression/preferences_dirty_guard.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| calls | [custom_theme_json](/crates/oxide-app/tests/regression/preferences_dirty_guard/custom_theme_json.md) |
