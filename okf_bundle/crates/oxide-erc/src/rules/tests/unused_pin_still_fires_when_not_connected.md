---
okf_version: "0.2"
type: Function
title: unused_pin_still_fires_when_not_connected
description: "[test]"
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/unused_pin_still_fires_when_not_connected
language: rust
---

# unused_pin_still_fires_when_not_connected

[test]

## Signature

```rust
fn unused_pin_still_fires_when_not_connected()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 178–189 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
| calls | [ctx](/crates/oxide-erc/src/rules/tests/ctx.md) |
| calls | [unused_pin](/crates/oxide-erc/src/rules/mod/unused_pin.md) |
