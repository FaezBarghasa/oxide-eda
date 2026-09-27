---
okf_version: "0.2"
type: Function
title: parse_pin_type
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/parse_pin_type
language: rust
---

# parse_pin_type

## Signature

```rust
fn parse_pin_type(s: &str) -> Option<PinDirection>
```

## Source
Lines 461–479 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [normalize](/crates/oxide-erc-dsl/src/compiler/normalize.md) |
| called_by | [eval_helper](/crates/oxide-erc-dsl/src/compiler/eval_helper.md) |
