---
okf_version: "0.2"
type: Function
title: resolve_return_path_rule
description: "Resolve [`ReturnPathRule`] for a given net class."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_return_path_rule
language: rust
---

# resolve_return_path_rule

Resolve [`ReturnPathRule`] for a given net class.

## Signature

```rust
impl ConstraintManager { pub fn resolve_return_path_rule(&self, net_class: &str) -> Option<&ReturnPathRule> }
```

## Visibility

- `pub`

## Docstring

Resolve [`ReturnPathRule`] for a given net class.

## Source
Lines 292–297 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
