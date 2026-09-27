---
okf_version: "0.2"
type: Function
title: pin_type_name
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/pin_type_name
language: rust
---

# pin_type_name

## Signature

```rust
fn pin_type_name(t: PinDirection) -> &'static str
```

## Source
Lines 481–498 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| called_by | [resolve_field](/crates/oxide-erc-dsl/src/compiler/resolve_field.md) |
