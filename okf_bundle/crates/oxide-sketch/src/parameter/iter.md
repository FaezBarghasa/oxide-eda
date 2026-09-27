---
okf_version: "0.2"
type: Function
title: iter
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/iter
language: rust
---

# iter

## Signature

```rust
impl ParameterTable { pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> }
```

## Visibility

- `pub`

## Source
Lines 50–52 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
