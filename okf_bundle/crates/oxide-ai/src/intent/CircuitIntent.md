---
okf_version: "0.2"
type: Class
title: CircuitIntent
description: Structured engineering intent extracted from natural language prompts.
resource: crates/oxide-ai/src/intent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:11:07Z"
concept_id: crates/oxide-ai/src/intent/CircuitIntent
language: rust
---

# CircuitIntent

Structured engineering intent extracted from natural language prompts.

## Signature

```rust
pub struct CircuitIntent
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Structured engineering intent extracted from natural language prompts.
[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `prompt`
- `components`
- `connections`
- `constraints`
- `is_validated`

## Source
Lines 62–74 in `crates/oxide-ai/src/intent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [intent](/crates/oxide-ai/src/intent.md) |
