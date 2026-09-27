---
okf_version: "0.2"
type: Class
title: ConnectionIntent
description: A connection intent linking two component pins with a named net.
resource: crates/oxide-ai/src/intent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:11:07Z"
concept_id: crates/oxide-ai/src/intent/ConnectionIntent
language: rust
---

# ConnectionIntent

A connection intent linking two component pins with a named net.

## Signature

```rust
pub struct ConnectionIntent
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A connection intent linking two component pins with a named net.
[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Methods

- `net_name`
- `from`
- `to`

## Source
Lines 51–58 in `crates/oxide-ai/src/intent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [intent](/crates/oxide-ai/src/intent.md) |
