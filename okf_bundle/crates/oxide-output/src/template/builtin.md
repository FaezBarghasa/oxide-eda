---
okf_version: "0.2"
type: Module
title: builtin
description: "Built-in sheet templates — ISO A0-A5 + ANSI A-E, portrait + landscape"
resource: crates/oxide-output/src/template/builtin.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/builtin
language: rust
---

# builtin

Built-in sheet templates — ISO A0-A5 + ANSI A-E, portrait + landscape

## Docstring

Built-in sheet templates — ISO A0-A5 + ANSI A-E, portrait + landscape
where standard practice allows. Constructed from a single
`standard_title_block()` helper so all 17 templates share identical
title-block layout (width, field positions, fonts) — only the page
size and orientation differ.

When users want to customise, they clone a built-in via the future
`.snxsht` parser (`format.rs`).

## Relationships

| Type | Target |
|------|--------|
| related | [all_builtin_ids](/crates/oxide-output/src/template/builtin/all_builtin_ids.md) |
| related | [load_builtin](/crates/oxide-output/src/template/builtin/load_builtin.md) |
| related | [standard_title_block](/crates/oxide-output/src/template/builtin/standard_title_block.md) |
| related | [has_expected_builtin_count](/crates/oxide-output/src/template/builtin/has_expected_builtin_count.md) |
| related | [every_id_loads](/crates/oxide-output/src/template/builtin/every_id_loads.md) |
| related | [unknown_id_returns_none](/crates/oxide-output/src/template/builtin/unknown_id_returns_none.md) |
| related | [default_template_id_loads](/crates/oxide-output/src/template/builtin/default_template_id_loads.md) |
| related | [all_title_blocks_carry_substitution_tokens](/crates/oxide-output/src/template/builtin/all_title_blocks_carry_substitution_tokens.md) |
