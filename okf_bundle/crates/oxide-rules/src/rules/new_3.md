---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/new_3
language: rust
---

# new

## Signature

```rust
pub fn new(
        scope: RuleScope,
        min_width: Microns,
        preferred_width: Microns,
        max_width: Microns,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 55–67 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
