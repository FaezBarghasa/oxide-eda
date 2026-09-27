---
okf_version: "0.2"
type: Function
title: add_rule
description: Add a design rule to the manager.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/add_rule_1
language: rust
---

# add_rule

Add a design rule to the manager.

## Signature

```rust
pub fn add_rule(&mut self, rule: DesignRule)
```

## Visibility

- `pub`

## Docstring

Add a design rule to the manager.

## Source
Lines 30–32 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
