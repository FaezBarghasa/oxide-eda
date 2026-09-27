---
okf_version: "0.2"
type: Function
title: seed_broken
description: "Put malformed JSON at the shared prefs path. `create_dir_all`"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/seed_broken_1
language: rust
---

# seed_broken

Put malformed JSON at the shared prefs path. `create_dir_all`

## Signature

```rust
fn seed_broken(&self)
```

## Docstring

Put malformed JSON at the shared prefs path. `create_dir_all`
first: on a fresh test config root the `oxide` directory only
comes into being when a writer creates it.

## Source
Lines 94–100 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
