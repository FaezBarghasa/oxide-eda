---
okf_version: "0.2"
type: Function
title: read_erc_severity_overrides
description: Read ERC severity overrides from preferences file. Returns an empty
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/read_erc_severity_overrides
language: rust
---

# read_erc_severity_overrides

Read ERC severity overrides from preferences file. Returns an empty

## Signature

```rust
pub fn read_erc_severity_overrides() -> std::collections::HashMap<oxide_erc::RuleKind, oxide_erc::Severity>
```

## Visibility

- `pub`

## Docstring

Read ERC severity overrides from preferences file. Returns an empty
map if the file is absent or the key missing — callers treat "no
entry" as "use the rule's default severity", matching the ui_state
semantic used throughout the app.

## Source
Lines 9–32 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| calls | [parse_erc_rule_kind](/crates/oxide-app/src/fonts/erc/parse_erc_rule_kind.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
