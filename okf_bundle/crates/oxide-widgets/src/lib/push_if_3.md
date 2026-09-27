---
okf_version: "0.2"
type: Function
title: push_if
resource: crates/oxide-widgets/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/lib/push_if_3
language: rust
---

# push_if

## Signature

```rust
fn push_if(self, cond: bool, f: impl FnOnce() -> E) -> Self
```

## Type Parameters

- `E: Into<Element<'a, M`

## Source
Lines 45–47 in `crates/oxide-widgets/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-widgets/src/lib.md) |
