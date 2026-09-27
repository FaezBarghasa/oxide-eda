---
okf_version: "0.2"
type: Function
title: default_true_bool
description: "Helper for `#[serde(default = \"...\")]` on bool fields that should"
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/default_true_bool
language: rust
---

# default_true_bool

Helper for `#[serde(default = "...")]` on bool fields that should

## Signature

```rust
fn default_true_bool() -> bool
```

## Docstring

Helper for `#[serde(default = "...")]` on bool fields that should
default to `true`. `bool::default()` is `false`, so this is needed
for fields where omission means "yes / enabled".

## Source
Lines 248–250 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
