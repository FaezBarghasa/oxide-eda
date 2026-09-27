---
okf_version: "0.2"
type: Function
title: to_binary
description: Converts a 12-state value to standard binary logic if resolvable.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/to_binary
language: rust
---

# to_binary

Converts a 12-state value to standard binary logic if resolvable.

## Signature

```rust
impl Logic12State { pub fn to_binary(self) -> Option<bool> }
```

## Visibility

- `pub`

## Docstring

Converts a 12-state value to standard binary logic if resolvable.

## Source
Lines 43–49 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
