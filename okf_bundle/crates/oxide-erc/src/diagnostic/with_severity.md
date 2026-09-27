---
okf_version: "0.2"
type: Function
title: with_severity
description: Override the severity (used by rules that hard-code a specific level
resource: crates/oxide-erc/src/diagnostic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/diagnostic/with_severity
language: rust
---

# with_severity

Override the severity (used by rules that hard-code a specific level

## Signature

```rust
impl Diagnostic { pub fn with_severity(mut self, severity: Severity) -> Self }
```

## Visibility

- `pub`

## Docstring

Override the severity (used by rules that hard-code a specific level
different from the kind's default, e.g. MissingPowerFlag → Info).

## Source
Lines 44–47 in `crates/oxide-erc/src/diagnostic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostic](/crates/oxide-erc/src/diagnostic.md) |
