---
okf_version: "0.2"
type: Function
title: emit_template
description: "Render a `Template` to its `.snxsht` string form. Currently a stub"
resource: crates/oxide-output/src/template/format.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/template/format/emit_template
language: rust
---

# emit_template

Render a `Template` to its `.snxsht` string form. Currently a stub

## Signature

```rust
pub fn emit_template(_template: &Template) -> String
```

## Visibility

- `pub`

## Docstring

Render a `Template` to its `.snxsht` string form. Currently a stub
that returns an empty string — the matching parser is a no-op so
round-trips are not meaningful until the format is reimplemented.

## Source
Lines 38–40 in `crates/oxide-output/src/template/format.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-output/src/template/format.md) |
