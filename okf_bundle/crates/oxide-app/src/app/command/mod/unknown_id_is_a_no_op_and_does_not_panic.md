---
okf_version: "0.2"
type: Function
title: unknown_id_is_a_no_op_and_does_not_panic
description: "An id with no catalog entry at all is untrusted input, not a"
resource: crates/oxide-app/src/app/command/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command/mod/unknown_id_is_a_no_op_and_does_not_panic
language: rust
---

# unknown_id_is_a_no_op_and_does_not_panic

An id with no catalog entry at all is untrusted input, not a

## Signature

```rust
fn unknown_id_is_a_no_op_and_does_not_panic()
```

## Decorators

- `test`

## Docstring

An id with no catalog entry at all is untrusted input, not a
programmer error — it must resolve to a no-op, never panic.
[test]

## Source
Lines 73–80 in `crates/oxide-app/src/app/command/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-app/src/app/command/mod.md) |
