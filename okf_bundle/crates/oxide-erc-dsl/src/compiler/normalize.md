---
okf_version: "0.2"
type: Function
title: normalize
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/normalize
language: rust
---

# normalize

## Signature

```rust
fn normalize(s: &str) -> String
```

## Source
Lines 500–505 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| called_by | [eval_field_cmp](/crates/oxide-erc-dsl/src/compiler/eval_field_cmp.md) |
| called_by | [eval_helper](/crates/oxide-erc-dsl/src/compiler/eval_helper.md) |
| called_by | [parse_pin_type](/crates/oxide-erc-dsl/src/compiler/parse_pin_type.md) |
