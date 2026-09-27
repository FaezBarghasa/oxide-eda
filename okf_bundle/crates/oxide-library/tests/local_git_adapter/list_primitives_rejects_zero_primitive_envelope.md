---
okf_version: "0.2"
type: Function
title: list_primitives_rejects_zero_primitive_envelope
description: "A well-formed envelope carrying *zero* primitives is a corrupt"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/list_primitives_rejects_zero_primitive_envelope
language: rust
---

# list_primitives_rejects_zero_primitive_envelope

A well-formed envelope carrying *zero* primitives is a corrupt

## Signature

```rust
fn list_primitives_rejects_zero_primitive_envelope()
```

## Decorators

- `test`

## Docstring

A well-formed envelope carrying *zero* primitives is a corrupt
library, not an empty one: the listing must fail loudly with the
`empty .snxfpt <file>` / `empty .snxsim <file>` diagnostic rather
than silently dropping the entry.
[test]

## Source
Lines 489–540 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
