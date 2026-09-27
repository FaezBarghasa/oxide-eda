---
okf_version: "0.2"
type: Function
title: is_low_confidence
description: Whether the parent UI should warn the user. Mirrors the 0.5
resource: crates/oxide-app/src/library/editor/symbol/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/ai_stub/is_low_confidence
language: rust
---

# is_low_confidence

Whether the parent UI should warn the user. Mirrors the 0.5

## Signature

```rust
impl AiPinoutPreview { pub fn is_low_confidence(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether the parent UI should warn the user. Mirrors the 0.5
threshold called out in `oxide-library/src/ai_stub.rs`.

## Source
Lines 16–18 in `crates/oxide-app/src/library/editor/symbol/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-app/src/library/editor/symbol/ai_stub.md) |
