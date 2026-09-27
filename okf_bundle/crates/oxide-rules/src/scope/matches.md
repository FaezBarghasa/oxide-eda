---
okf_version: "0.2"
type: Function
title: matches
description: Check if this scope matches the query parameters.
resource: crates/oxide-rules/src/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:31:50Z"
concept_id: crates/oxide-rules/src/scope/matches
language: rust
---

# matches

Check if this scope matches the query parameters.

## Signature

```rust
impl RuleScope { pub fn matches(&self, net: &str, net_class: Option<&str>, room: Option<&str>) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if this scope matches the query parameters.

## Source
Lines 107–114 in `crates/oxide-rules/src/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-rules/src/scope.md) |
