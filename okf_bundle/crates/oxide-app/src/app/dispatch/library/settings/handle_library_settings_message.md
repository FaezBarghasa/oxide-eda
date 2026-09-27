---
okf_version: "0.2"
type: Function
title: handle_library_settings_message
resource: crates/oxide-app/src/app/dispatch/library/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/settings/handle_library_settings_message
language: rust
---

# handle_library_settings_message

## Signature

```rust
impl Oxide { pub(super) fn handle_library_settings_message(&mut self, msg: SettingsMsg) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 10–198 in `crates/oxide-app/src/app/dispatch/library/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/dispatch/library/settings.md) |
| calls | [read_env_credentials](/crates/oxide-app/src/library/settings/digikey_oauth/read_env_credentials.md) |
| calls | [run_blocking](/crates/oxide-app/src/library/settings/digikey_oauth/run_blocking.md) |
