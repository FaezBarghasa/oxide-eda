---
okf_version: "0.2"
type: Function
title: restore
description: "Put `path` back to `before` — content restored, or removed again when"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/restore
language: rust
---

# restore

Put `path` back to `before` — content restored, or removed again when

## Signature

```rust
fn restore(path: &std::path::Path, before: Option<&Vec<u8>>) -> std::io::Result<()>
```

## Docstring

Put `path` back to `before` — content restored, or removed again when
it did not exist. A `NotFound` on the removal is already the state
being asked for.

## Source
Lines 106–114 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| called_by | [drop](/crates/oxide-app/tests/regression/preferences_prefs_recovery/drop.md) |
