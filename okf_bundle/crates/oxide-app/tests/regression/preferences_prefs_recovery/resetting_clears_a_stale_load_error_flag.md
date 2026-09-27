---
okf_version: "0.2"
type: Function
title: resetting_clears_a_stale_load_error_flag
description: "The banner is state, and the action has to clear it — otherwise a user"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_clears_a_stale_load_error_flag
language: rust
---

# resetting_clears_a_stale_load_error_flag

The banner is state, and the action has to clear it — otherwise a user

## Signature

```rust
fn resetting_clears_a_stale_load_error_flag()
```

## Decorators

- `test`

## Docstring

The banner is state, and the action has to clear it — otherwise a user
who resets is left staring at the same warning with no way to dismiss
it. Pre-set the flag to a stale value the way a boot-time failure
would, then drive the message the button emits.
[test]

## Source
Lines 192–213 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
