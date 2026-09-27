---
okf_version: "0.2"
type: Function
title: default_severity
description: Default severity. Users can override per-rule in the Preferences
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/default_severity
language: rust
---

# default_severity

Default severity. Users can override per-rule in the Preferences

## Signature

```rust
impl RuleKind { pub fn default_severity(self) -> Severity }
```

## Visibility

- `pub`

## Docstring

Default severity. Users can override per-rule in the Preferences
panel via `ui_state.erc_severity_override`.

## Source
Lines 87–102 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
