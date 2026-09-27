---
okf_version: "0.2"
type: Function
title: resolve_high_speed_rule
description: Resolve high-speed routing rule for a given net class.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_high_speed_rule_1
language: rust
---

# resolve_high_speed_rule

Resolve high-speed routing rule for a given net class.

## Signature

```rust
pub fn resolve_high_speed_rule(&self, net_class: &str) -> Option<&HighSpeedRule>
```

## Visibility

- `pub`

## Docstring

Resolve high-speed routing rule for a given net class.

## Source
Lines 121–126 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
