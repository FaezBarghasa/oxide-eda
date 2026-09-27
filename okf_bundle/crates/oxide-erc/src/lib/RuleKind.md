---
okf_version: "0.2"
type: Class
title: RuleKind
description: What kind of violation this is. Stable identifier that maps to a severity
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/RuleKind
language: rust
---

# RuleKind

What kind of violation this is. Stable identifier that maps to a severity

## Signature

```rust
pub enum RuleKind
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

What kind of violation this is. Stable identifier that maps to a severity
in the user's configuration. Ordered by the Altium ERC matrix conventions.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 35–64 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
