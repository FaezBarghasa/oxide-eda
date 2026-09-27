---
okf_version: "0.2"
type: Function
title: reporting_the_reset_outcome_does_not_mark_the_dialog_dirty
description: "The status line is feedback about one action, not an edit — reporting"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/reporting_the_reset_outcome_does_not_mark_the_dialog_dirty
language: rust
---

# reporting_the_reset_outcome_does_not_mark_the_dialog_dirty

The status line is feedback about one action, not an edit — reporting

## Signature

```rust
fn reporting_the_reset_outcome_does_not_mark_the_dialog_dirty()
```

## Decorators

- `test`

## Docstring

The status line is feedback about one action, not an edit — reporting
it must never trip the unsaved-changes guard, or the user is trapped in
a dialog that refuses to close. Same rule as the export status lines.
[test]

## Source
Lines 299–327 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
