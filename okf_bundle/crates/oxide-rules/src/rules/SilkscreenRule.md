---
okf_version: "0.2"
type: Class
title: SilkscreenRule
description: Silkscreen to solder mask and silkscreen to silkscreen clearance rules.
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/SilkscreenRule
language: rust
---

# SilkscreenRule

Silkscreen to solder mask and silkscreen to silkscreen clearance rules.

## Signature

```rust
pub struct SilkscreenRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Silkscreen to solder mask and silkscreen to silkscreen clearance rules.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `scope`
- `min_clearance_to_mask`
- `min_clearance_to_silk`
- `min_line_width`

## Source
Lines 129–137 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
