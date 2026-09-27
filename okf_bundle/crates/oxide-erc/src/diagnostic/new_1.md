---
okf_version: "0.2"
type: Function
title: new
description: "Shorthand for built-in rules: severity comes from the kind's default."
resource: crates/oxide-erc/src/diagnostic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/diagnostic/new_1
language: rust
---

# new

Shorthand for built-in rules: severity comes from the kind's default.

## Signature

```rust
pub fn new(kind: RuleKind, message: impl Into<String>, location: Point) -> Self
```

## Visibility

- `pub`

## Docstring

Shorthand for built-in rules: severity comes from the kind's default.

## Source
Lines 30–40 in `crates/oxide-erc/src/diagnostic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostic](/crates/oxide-erc/src/diagnostic.md) |
