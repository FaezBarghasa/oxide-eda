---
okf_version: "0.2"
type: Function
title: add_connection
description: Add a connection intent.
resource: crates/oxide-ai/src/intent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:11:07Z"
concept_id: crates/oxide-ai/src/intent/add_connection
language: rust
---

# add_connection

Add a connection intent.

## Signature

```rust
impl CircuitIntent { pub fn add_connection(
        &mut self,
        net_name: impl Into<String>,
        from: (impl Into<String>, impl Into<String>),
        to: (impl Into<String>, impl Into<String>),
    ) }
```

## Visibility

- `pub`

## Docstring

Add a connection intent.

## Source
Lines 93–104 in `crates/oxide-ai/src/intent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [intent](/crates/oxide-ai/src/intent.md) |
