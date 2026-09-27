---
okf_version: "0.2"
type: Function
title: lookup
description: "Look up a single token. Returns `None` for unknown tokens — the"
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution/lookup
language: rust
---

# lookup

Look up a single token. Returns `None` for unknown tokens — the

## Signature

```rust
impl SubstitutionContext<'a> { fn lookup(&self, token: &str) -> Option<String> }
```

## Type Parameters

- `'a`

## Docstring

Look up a single token. Returns `None` for unknown tokens — the
resolver renders that as an empty string.

## Source
Lines 43–78 in `crates/oxide-output/src/substitution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [substitution](/crates/oxide-output/src/substitution.md) |
