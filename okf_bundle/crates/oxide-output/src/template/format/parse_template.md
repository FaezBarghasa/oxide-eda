---
okf_version: "0.2"
type: Function
title: parse_template
description: "Parse a `.snxsht` source string into a `Template`. Currently always"
resource: crates/oxide-output/src/template/format.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/template/format/parse_template
language: rust
---

# parse_template

Parse a `.snxsht` source string into a `Template`. Currently always

## Signature

```rust
pub fn parse_template(_source: &str, _fallback_id: &str) -> Result<Template, SnxshtError>
```

## Visibility

- `pub`

## Docstring

Parse a `.snxsht` source string into a `Template`. Currently always
returns `SnxshtError::NotImplemented` — see module docs.

## Source
Lines 31–33 in `crates/oxide-output/src/template/format.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-output/src/template/format.md) |
