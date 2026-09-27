---
okf_version: "0.2"
type: Function
title: starts_with_ascii_ci
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/starts_with_ascii_ci
language: rust
---

# starts_with_ascii_ci

## Signature

```rust
fn starts_with_ascii_ci(haystack: &[u8], start: usize, needle: &[u8]) -> bool
```

## Source
Lines 493–501 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| called_by | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
