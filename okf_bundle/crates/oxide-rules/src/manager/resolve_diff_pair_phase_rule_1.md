---
okf_version: "0.2"
type: Function
title: resolve_diff_pair_phase_rule
description: "Resolve [`DiffPairPhaseRule`] for a given net class."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/resolve_diff_pair_phase_rule_1
language: rust
---

# resolve_diff_pair_phase_rule

Resolve [`DiffPairPhaseRule`] for a given net class.

## Signature

```rust
pub fn resolve_diff_pair_phase_rule(&self, net_class: &str) -> Option<&DiffPairPhaseRule>
```

## Visibility

- `pub`

## Docstring

Resolve [`DiffPairPhaseRule`] for a given net class.

## Source
Lines 432–437 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
