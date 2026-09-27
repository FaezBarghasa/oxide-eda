---
okf_version: "0.2"
type: Function
title: write_pin_matrix_overrides
description: Persist pin-connection matrix overrides.
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/write_pin_matrix_overrides
language: rust
---

# write_pin_matrix_overrides

Persist pin-connection matrix overrides.

## Signature

```rust
pub fn write_pin_matrix_overrides(
    overrides: &std::collections::HashMap<(u8, u8), oxide_erc::Severity>,
)
```

## Visibility

- `pub`

## Docstring

Persist pin-connection matrix overrides.

## Source
Lines 141–154 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [erc_severity_key](/crates/oxide-app/src/fonts/erc/erc_severity_key.md) |
| called_by | [dispatch_erc_message](/crates/oxide-app/src/app/dispatch/overlay/dispatch_erc_message.md) |
