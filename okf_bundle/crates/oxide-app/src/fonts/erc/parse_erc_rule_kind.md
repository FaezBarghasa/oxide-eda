---
okf_version: "0.2"
type: Function
title: parse_erc_rule_kind
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/parse_erc_rule_kind
language: rust
---

# parse_erc_rule_kind

## Signature

```rust
fn parse_erc_rule_kind(s: &str) -> Option<oxide_erc::RuleKind>
```

## Source
Lines 70–87 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| called_by | [read_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/read_erc_severity_overrides.md) |
