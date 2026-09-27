---
okf_version: "0.2"
type: Function
title: classify_pin_kind
description: Classify a pin by name + description keywords.
resource: crates/oxide-library/src/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/ai_stub/classify_pin_kind
language: rust
---

# classify_pin_kind

Classify a pin by name + description keywords.

## Signature

```rust
fn classify_pin_kind(name: &str, description: &str) -> String
```

## Docstring

Classify a pin by name + description keywords.

## Source
Lines 102–149 in `crates/oxide-library/src/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/src/ai_stub.md) |
| called_by | [parse_pin_row](/crates/oxide-library/src/ai_stub/parse_pin_row.md) |
