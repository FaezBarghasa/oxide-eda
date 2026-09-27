---
okf_version: "0.2"
type: Class
title: PrefsPathGuard
description: Snapshots the shared config files these tests can disturb and puts
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/PrefsPathGuard
language: rust
---

# PrefsPathGuard

Snapshots the shared config files these tests can disturb and puts

## Signature

```rust
struct PrefsPathGuard
```

## Docstring

Snapshots the shared config files these tests can disturb and puts
them back when it goes out of scope — on the assertion-failure path
too, which is why this is a `Drop` guard and not a tail call at the end
of the test body.

The keymap file is in here because `PrefMsg::Save` persists the keymap
working copy as well as the prefs, so the one test that drives a real
Save would otherwise leave `keyboard_shortcuts.toml` behind for every
later `Oxide::new()` in this binary to load.

## Methods

- `path`
- `before`
- `keymap_path`
- `keymap_before`

## Source
Lines 70–75 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
