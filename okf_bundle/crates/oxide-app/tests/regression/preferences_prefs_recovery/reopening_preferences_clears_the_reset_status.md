---
okf_version: "0.2"
type: Function
title: reopening_preferences_clears_the_reset_status
description: "Reopening the dialog is a fresh session for transient feedback: the"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/reopening_preferences_clears_the_reset_status
language: rust
---

# reopening_preferences_clears_the_reset_status

Reopening the dialog is a fresh session for transient feedback: the

## Signature

```rust
fn reopening_preferences_clears_the_reset_status()
```

## Decorators

- `test`

## Docstring

Reopening the dialog is a fresh session for transient feedback: the
last reset's result must not still be sitting in the banner.
[test]

## Source
Lines 332–354 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
