---
okf_version: "0.2"
type: Function
title: resolve_net_antenna_rule
description: "Resolve the most specific [`NetAntennaRule`] for a given net, net class, and room."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_net_antenna_rule
language: rust
---

# resolve_net_antenna_rule

Resolve the most specific [`NetAntennaRule`] for a given net, net class, and room.

## Signature

```rust
impl ConstraintManager { pub fn resolve_net_antenna_rule(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
    ) -> Option<&NetAntennaRule> }
```

## Visibility

- `pub`

## Docstring

Resolve the most specific [`NetAntennaRule`] for a given net, net class, and room.

## Source
Lines 272–289 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
