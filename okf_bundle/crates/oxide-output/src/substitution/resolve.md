---
okf_version: "0.2"
type: Function
title: resolve
description: "Replace every `${IDENT}` in `input` with its resolved value. Unknown"
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution/resolve
language: rust
---

# resolve

Replace every `${IDENT}` in `input` with its resolved value. Unknown

## Signature

```rust
pub fn resolve(input: &str, ctx: &SubstitutionContext<'_>) -> String
```

## Visibility

- `pub`

## Docstring

Replace every `${IDENT}` in `input` with its resolved value. Unknown
tokens render as empty string; non-token `${...}` strings pass through.

## Source
Lines 83–102 in `crates/oxide-output/src/substitution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [substitution](/crates/oxide-output/src/substitution.md) |
| calls | [scan_token](/crates/oxide-output/src/substitution/scan_token.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
