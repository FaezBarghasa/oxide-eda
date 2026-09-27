---
okf_version: "0.2"
type: Function
title: resolve_component_clearance_rule
description: "Resolve the most specific [`ComponentClearanceRule`] for a given component/room."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_component_clearance_rule_1
language: rust
---

# resolve_component_clearance_rule

Resolve the most specific [`ComponentClearanceRule`] for a given component/room.

## Signature

```rust
pub fn resolve_component_clearance_rule(
        &self,
        component_id: &str,
        room: Option<&str>,
    ) -> Option<&ComponentClearanceRule>
```

## Visibility

- `pub`

## Docstring

Resolve the most specific [`ComponentClearanceRule`] for a given component/room.

## Source
Lines 391–409 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
