---
okf_version: "0.2"
type: Function
title: resetting_a_healthy_prefs_file_moves_nothing_and_says_so
description: The guard is the whole reason the recovery action is safe to run beside
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_a_healthy_prefs_file_moves_nothing_and_says_so
language: rust
---

# resetting_a_healthy_prefs_file_moves_nothing_and_says_so

The guard is the whole reason the recovery action is safe to run beside

## Signature

```rust
fn resetting_a_healthy_prefs_file_moves_nothing_and_says_so()
```

## Decorators

- `test`

## Docstring

The guard is the whole reason the recovery action is safe to run beside
the other tests, and it is also the behaviour that protects a user who
repaired the file by hand between the banner being painted and the
click. A refactor that drops it renames a healthy prefs.json away —
#594 again.
[test]

## Source
Lines 154–185 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
