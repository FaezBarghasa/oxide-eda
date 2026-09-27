---
okf_version: "0.2"
type: Function
title: generate_circuit_generation_prompt
description: "Generate a strict JSON schema prompt instructing the LLM to output a valid `CircuitIntent`."
resource: crates/oxide-ai/src/prompt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:11:21Z"
concept_id: crates/oxide-ai/src/prompt/generate_circuit_generation_prompt
language: rust
---

# generate_circuit_generation_prompt

Generate a strict JSON schema prompt instructing the LLM to output a valid `CircuitIntent`.

## Signature

```rust
pub fn generate_circuit_generation_prompt(user_prompt: &str) -> String
```

## Visibility

- `pub`

## Docstring

Generate a strict JSON schema prompt instructing the LLM to output a valid `CircuitIntent`.

## Source
Lines 4–38 in `crates/oxide-ai/src/prompt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prompt](/crates/oxide-ai/src/prompt.md) |
| called_by | [test_prompt_generation](/crates/oxide-ai/tests/ai_tests/test_prompt_generation.md) |
