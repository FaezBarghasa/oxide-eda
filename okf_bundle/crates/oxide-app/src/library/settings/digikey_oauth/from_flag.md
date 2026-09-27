---
okf_version: "0.2"
type: Function
title: from_flag
description: Wrap an existing flag — used by the dispatcher so the UI can
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/from_flag
language: rust
---

# from_flag

Wrap an existing flag — used by the dispatcher so the UI can

## Signature

```rust
impl CancelHandle { pub fn from_flag(flag: Arc<AtomicBool>) -> Self }
```

## Visibility

- `pub`

## Docstring

Wrap an existing flag — used by the dispatcher so the UI can
hold the same `AtomicBool` it later mutates from the Cancel
button.

## Source
Lines 81–83 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
