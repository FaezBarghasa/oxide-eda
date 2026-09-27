---
okf_version: "0.2"
type: Function
title: load_builtin
description: "Load a built-in template by ID. Returns `None` for unknown IDs."
resource: crates/oxide-output/src/template/builtin.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/builtin/load_builtin
language: rust
---

# load_builtin

Load a built-in template by ID. Returns `None` for unknown IDs.

## Signature

```rust
pub fn load_builtin(id: &TemplateId) -> Option<Template>
```

## Visibility

- `pub`

## Docstring

Load a built-in template by ID. Returns `None` for unknown IDs.

## Source
Lines 22–34 in `crates/oxide-output/src/template/builtin.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builtin](/crates/oxide-output/src/template/builtin.md) |
| calls | [standard_title_block](/crates/oxide-output/src/template/builtin/standard_title_block.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
| called_by | [all_title_blocks_carry_substitution_tokens](/crates/oxide-output/src/template/builtin/all_title_blocks_carry_substitution_tokens.md) |
| called_by | [default_template_id_loads](/crates/oxide-output/src/template/builtin/default_template_id_loads.md) |
| called_by | [every_id_loads](/crates/oxide-output/src/template/builtin/every_id_loads.md) |
