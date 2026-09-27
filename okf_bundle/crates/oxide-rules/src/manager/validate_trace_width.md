---
okf_version: "0.2"
type: Function
title: validate_trace_width
description: Validate that a routed trace width conforms to the hierarchical width rule.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_trace_width
language: rust
---

# validate_trace_width

Validate that a routed trace width conforms to the hierarchical width rule.

## Signature

```rust
impl ConstraintManager { pub fn validate_trace_width(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        actual_width: Microns,
    ) -> Result<(), RuleViolation> }
```

## Visibility

- `pub`

## Docstring

Validate that a routed trace width conforms to the hierarchical width rule.
[allow(clippy::result_large_err)]

## Source
Lines 130–148 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
