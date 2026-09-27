---
okf_version: "0.2"
type: Function
title: ctx
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution/ctx
language: rust
---

# ctx

## Signature

```rust
fn ctx(m: &'a ProjectMetadata) -> SubstitutionContext<'a>
```

## Type Parameters

- `'a`

## Source
Lines 161–174 in `crates/oxide-output/src/substitution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [substitution](/crates/oxide-output/src/substitution.md) |
| called_by | [document_number_falls_back_to_custom_field](/crates/oxide-output/src/substitution/document_number_falls_back_to_custom_field.md) |
| called_by | [dollar_without_brace_passes_through](/crates/oxide-output/src/substitution/dollar_without_brace_passes_through.md) |
| called_by | [malformed_passes_through](/crates/oxide-output/src/substitution/malformed_passes_through.md) |
| called_by | [multiline_text](/crates/oxide-output/src/substitution/multiline_text.md) |
| called_by | [multiple_tokens_same_line](/crates/oxide-output/src/substitution/multiple_tokens_same_line.md) |
| called_by | [plain_text_unchanged](/crates/oxide-output/src/substitution/plain_text_unchanged.md) |
| called_by | [preserves_utf8](/crates/oxide-output/src/substitution/preserves_utf8.md) |
| called_by | [resolves_builtin_tokens](/crates/oxide-output/src/substitution/resolves_builtin_tokens.md) |
| called_by | [resolves_custom_fields](/crates/oxide-output/src/substitution/resolves_custom_fields.md) |
| called_by | [sheet_number_token_drops_when_toggle_off](/crates/oxide-output/src/substitution/sheet_number_token_drops_when_toggle_off.md) |
| called_by | [unknown_token_renders_empty](/crates/oxide-output/src/substitution/unknown_token_renders_empty.md) |
| called_by | [variant_token_only_emits_when_physical_structure_on](/crates/oxide-output/src/substitution/variant_token_only_emits_when_physical_structure_on.md) |
