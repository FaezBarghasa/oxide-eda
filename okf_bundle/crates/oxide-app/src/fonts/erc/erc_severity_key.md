---
okf_version: "0.2"
type: Function
title: erc_severity_key
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/erc_severity_key
language: rust
---

# erc_severity_key

## Signature

```rust
fn erc_severity_key(sev: oxide_erc::Severity) -> &'static str
```

## Source
Lines 89–96 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| called_by | [write_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/write_erc_severity_overrides.md) |
| called_by | [write_pin_matrix_overrides](/crates/oxide-app/src/fonts/erc/write_pin_matrix_overrides.md) |
