---
okf_version: "0.2"
type: Class
title: PushIf
description: Conditional push — append an element only when a condition is true.
resource: crates/oxide-widgets/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/lib/PushIf
language: rust
---

# PushIf

Conditional push — append an element only when a condition is true.

## Signature

```rust
pub trait PushIf
```

## Type Parameters

- `'a`
- `M`

## Visibility

- `pub`

## Docstring

Conditional push — append an element only when a condition is true.

```rust,ignore
col.push_if(has_badge, || text(badge).size(10))
```

## Source
Lines 34–36 in `crates/oxide-widgets/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-widgets/src/lib.md) |
