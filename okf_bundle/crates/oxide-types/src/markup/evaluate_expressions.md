---
okf_version: "0.2"
type: Function
title: evaluate_expressions
description: Evaluate a subset of Altium-style expression variables.
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/evaluate_expressions
language: rust
---

# evaluate_expressions

Evaluate a subset of Altium-style expression variables.

## Signature

```rust
pub fn evaluate_expressions(input: &str, ctx: &ExpressionEvalContext<'_>) -> String
```

## Visibility

- `pub`

## Docstring

Evaluate a subset of Altium-style expression variables.

Supported:
- `${refdes:<key>}`
- `@{<name>}`
- `CELL()`
- `NET_NAME(<pin>)`

Unresolved expressions are preserved verbatim to avoid destructive output.

## Source
Lines 99–186 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [read_braced](/crates/oxide-types/src/markup/read_braced.md) |
| calls | [eval_dollar_expression](/crates/oxide-types/src/markup/eval_dollar_expression.md) |
| calls | [eval_at_expression](/crates/oxide-types/src/markup/eval_at_expression.md) |
| calls | [starts_with_ascii_ci](/crates/oxide-types/src/markup/starts_with_ascii_ci.md) |
| calls | [read_parenthesized](/crates/oxide-types/src/markup/read_parenthesized.md) |
| calls | [eval_net_name](/crates/oxide-types/src/markup/eval_net_name.md) |
| called_by | [normalize_standard_text](/crates/oxide-output/src/pdf/content/normalize_standard_text.md) |
| called_by | [normalize_standard_text_with_ctx](/crates/oxide-output/src/svg/mod/normalize_standard_text_with_ctx.md) |
| called_by | [evaluates_cell_and_net_name](/crates/oxide-types/src/markup/evaluates_cell_and_net_name.md) |
| called_by | [evaluates_refdes_and_at_variables](/crates/oxide-types/src/markup/evaluates_refdes_and_at_variables.md) |
| called_by | [unresolved_expressions_are_preserved](/crates/oxide-types/src/markup/unresolved_expressions_are_preserved.md) |
