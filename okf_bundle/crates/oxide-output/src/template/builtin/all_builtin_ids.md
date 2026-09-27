---
okf_version: "0.2"
type: Function
title: all_builtin_ids
description: "Every built-in template's IDs, in display order."
resource: crates/oxide-output/src/template/builtin.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/builtin/all_builtin_ids
language: rust
---

# all_builtin_ids

Every built-in template's IDs, in display order.

## Signature

```rust
pub fn all_builtin_ids() -> Vec<TemplateId>
```

## Visibility

- `pub`

## Docstring

Every built-in template's IDs, in display order.

## Source
Lines 14–19 in `crates/oxide-output/src/template/builtin.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builtin](/crates/oxide-output/src/template/builtin.md) |
| called_by | [all_title_blocks_carry_substitution_tokens](/crates/oxide-output/src/template/builtin/all_title_blocks_carry_substitution_tokens.md) |
| called_by | [every_id_loads](/crates/oxide-output/src/template/builtin/every_id_loads.md) |
