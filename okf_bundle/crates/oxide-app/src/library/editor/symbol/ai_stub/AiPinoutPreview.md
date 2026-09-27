---
okf_version: "0.2"
type: Class
title: AiPinoutPreview
description: "UI-facing wrapper around a future AI pinout guess. Holds `confidence`"
resource: crates/oxide-app/src/library/editor/symbol/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/ai_stub/AiPinoutPreview
language: rust
---

# AiPinoutPreview

UI-facing wrapper around a future AI pinout guess. Holds `confidence`

## Signature

```rust
pub struct AiPinoutPreview
```

## Decorators

- `derive(Debug, Clone, PartialEq, Default)`

## Visibility

- `pub`

## Docstring

UI-facing wrapper around a future AI pinout guess. Holds `confidence`
so the caller can warn when the heuristic is not reliable.
[derive(Debug, Clone, PartialEq, Default)]

## Methods

- `confidence`

## Source
Lines 9–11 in `crates/oxide-app/src/library/editor/symbol/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-app/src/library/editor/symbol/ai_stub.md) |
