---
okf_version: "0.2"
type: Function
title: consumes_a_non_copy_payload
description: "Taken by value, so a non-`Copy` payload is moved in and dropped"
resource: crates/oxide-app/src/ignore.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/ignore/consumes_a_non_copy_payload
language: rust
---

# consumes_a_non_copy_payload

Taken by value, so a non-`Copy` payload is moved in and dropped

## Signature

```rust
fn consumes_a_non_copy_payload()
```

## Decorators

- `test`

## Docstring

Taken by value, so a non-`Copy` payload is moved in and dropped
here — nothing leaks and no borrow is left behind.
[test]

## Source
Lines 59–62 in `crates/oxide-app/src/ignore.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ignore](/crates/oxide-app/src/ignore.md) |
