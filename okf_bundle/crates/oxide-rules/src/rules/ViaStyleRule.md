---
okf_version: "0.2"
type: Class
title: ViaStyleRule
description: Allowed via drill and pad dimensions per scope or via technology.
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/ViaStyleRule
language: rust
---

# ViaStyleRule

Allowed via drill and pad dimensions per scope or via technology.

## Signature

```rust
pub struct ViaStyleRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Allowed via drill and pad dimensions per scope or via technology.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `scope`
- `min_drill`
- `min_diameter`
- `preferred_drill`
- `preferred_diameter`

## Source
Lines 95–101 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
