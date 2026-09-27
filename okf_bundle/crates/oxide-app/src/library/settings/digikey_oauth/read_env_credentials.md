---
okf_version: "0.2"
type: Function
title: read_env_credentials
description: Read environment-supplied DigiKey credentials. Returns empty
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/read_env_credentials
language: rust
---

# read_env_credentials

Read environment-supplied DigiKey credentials. Returns empty

## Signature

```rust
pub fn read_env_credentials() -> (String, String)
```

## Visibility

- `pub`

## Docstring

Read environment-supplied DigiKey credentials. Returns empty
strings when unset — the caller treats empty client_id as "not
configured" and surfaces a clear failure.

## Source
Lines 404–408 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [handle_library_settings_message](/crates/oxide-app/src/app/dispatch/library/settings/handle_library_settings_message.md) |
