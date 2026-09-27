---
okf_version: "0.2"
type: Function
title: dispatch_preferences_message
description: "Preferences modal family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_preferences_message_1
language: rust
---

# dispatch_preferences_message

Preferences modal family handler (namespaced, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_preferences_message(&mut self, msg: PreferencesMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Preferences modal family handler (namespaced, ADR-0001 D3).

## Source
Lines 437–453 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
