---
okf_version: "0.2"
type: Function
title: pref_token
description: "Stable token used to persist this mode to `prefs.json`."
resource: crates/oxide-app/src/render_config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/render_config/pref_token
language: rust
---

# pref_token

Stable token used to persist this mode to `prefs.json`.

## Signature

```rust
impl PinSelectionMode { pub fn pref_token(self) -> &'static str }
```

## Visibility

- `pub`

## Docstring

Stable token used to persist this mode to `prefs.json`.

## Source
Lines 114–119 in `crates/oxide-app/src/render_config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config](/crates/oxide-app/src/render_config.md) |
