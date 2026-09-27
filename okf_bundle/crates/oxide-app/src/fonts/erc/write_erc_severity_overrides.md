---
okf_version: "0.2"
type: Function
title: write_erc_severity_overrides
description: Persist the ERC severity-override map. Stored as an object keyed by
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/write_erc_severity_overrides
language: rust
---

# write_erc_severity_overrides

Persist the ERC severity-override map. Stored as an object keyed by

## Signature

```rust
pub fn write_erc_severity_overrides(
    overrides: &std::collections::HashMap<oxide_erc::RuleKind, oxide_erc::Severity>,
)
```

## Visibility

- `pub`

## Docstring

Persist the ERC severity-override map. Stored as an object keyed by
rule name so the file stays human-readable when the user edits it by
hand.

## Source
Lines 37–50 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [erc_rule_kind_key](/crates/oxide-app/src/fonts/erc/erc_rule_kind_key.md) |
| calls | [erc_severity_key](/crates/oxide-app/src/fonts/erc/erc_severity_key.md) |
| called_by | [handle_erc_severity_changed](/crates/oxide-app/src/app/handlers/erc/modals/handle_erc_severity_changed.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
