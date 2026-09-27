---
okf_version: "0.2"
type: Function
title: parse
description: Parse DSL source text into a list of rule declarations.
resource: crates/oxide-erc-dsl/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/parser/parse
language: rust
---

# parse

Parse DSL source text into a list of rule declarations.

## Signature

```rust
pub fn parse(src: &str) -> Result<Vec<RuleAst>, Vec<(usize, String)>>
```

## Visibility

- `pub`

## Docstring

Parse DSL source text into a list of rule declarations.
Returns the list of parse errors (byte offsets) on failure.

## Source
Lines 52–66 in `crates/oxide-erc-dsl/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-erc-dsl/src/parser.md) |
| calls | [program_parser](/crates/oxide-erc-dsl/src/parser/program_parser.md) |
| calls | [span](/crates/oxide-app/src/renderer_scene_canvas/span.md) |
