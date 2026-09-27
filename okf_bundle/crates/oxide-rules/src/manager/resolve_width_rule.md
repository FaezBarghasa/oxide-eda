---
okf_version: "0.2"
type: Function
title: resolve_width_rule
description: "Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_width_rule
language: rust
---

# resolve_width_rule

Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room.

## Signature

```rust
impl ConstraintManager { pub fn resolve_width_rule(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
    ) -> Option<&WidthRule> }
```

## Visibility

- `pub`

## Docstring

Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room.

## Source
Lines 80–98 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
