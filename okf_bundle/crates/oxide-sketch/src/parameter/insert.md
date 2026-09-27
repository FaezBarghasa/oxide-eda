---
okf_version: "0.2"
type: Function
title: insert
description: Insert / overwrite a parameter source string.
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/insert
language: rust
---

# insert

Insert / overwrite a parameter source string.

## Signature

```rust
impl ParameterTable { pub fn insert(&mut self, name: impl Into<String>, src: impl Into<String>) }
```

## Visibility

- `pub`

## Docstring

Insert / overwrite a parameter source string.

## Source
Lines 42–44 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
