---
okf_version: "0.2"
type: Function
title: from_pref_token
description: Parse a persisted token back into a mode — unknown/legacy values
resource: crates/oxide-app/src/render_config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/render_config/from_pref_token
language: rust
---

# from_pref_token

Parse a persisted token back into a mode — unknown/legacy values

## Signature

```rust
impl PinSelectionMode { pub fn from_pref_token(s: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Parse a persisted token back into a mode — unknown/legacy values
fall back to the `PinOnly` default.

## Source
Lines 123–128 in `crates/oxide-app/src/render_config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config](/crates/oxide-app/src/render_config.md) |
