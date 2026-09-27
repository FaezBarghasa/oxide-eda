---
okf_version: "0.2"
type: Function
title: subject_ref
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/subject_ref
language: rust
---

# subject_ref

## Signature

```rust
fn subject_ref(subject: &'a Subject<'a>) -> Subject<'a>
```

## Type Parameters

- `'a`

## Source
Lines 275–282 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [Net](/crates/oxide-types/src/net/Net.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| called_by | [eval_expr](/crates/oxide-erc-dsl/src/compiler/eval_expr.md) |
