---
okf_version: "0.2"
type: Function
title: validate_return_path
description: Validate unbroken reference return path for high-speed net.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_return_path_1
language: rust
---

# validate_return_path

Validate unbroken reference return path for high-speed net.

## Signature

```rust
pub fn validate_return_path(
        &self,
        net: &str,
        net_class: &str,
        plane_net: &str,
        crosses_split: bool,
        location: Option<(f64, f64)>,
    ) -> Result<(), RuleViolation>
```

## Decorators

- `allow(clippy::result_large_err)`

## Visibility

- `pub`

## Docstring

Validate unbroken reference return path for high-speed net.
[allow(clippy::result_large_err)]

## Source
Lines 371–388 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
