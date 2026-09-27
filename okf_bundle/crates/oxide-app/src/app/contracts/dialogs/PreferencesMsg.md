---
okf_version: "0.2"
type: Class
title: PreferencesMsg
description: Preferences modal message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/PreferencesMsg
language: rust
---

# PreferencesMsg

Preferences modal message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum PreferencesMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Preferences modal message family (ADR-0001 D3). Namespaced under
`Message::Preferences` and routed to `dispatch_preferences_message`.
[derive(Debug, Clone)]

## Source
Lines 91–100 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
