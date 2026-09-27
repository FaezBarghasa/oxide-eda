---
okf_version: "0.2"
type: Module
title: substitution
description: "Text substitution — resolves `${TITLE}`, `${DATE}`, `${REV}`, etc."
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution
language: rust
---

# substitution

Text substitution — resolves `${TITLE}`, `${DATE}`, `${REV}`, etc.

## Docstring

Text substitution — resolves `${TITLE}`, `${DATE}`, `${REV}`, etc.

See `OUTPUT_PLAN.md` §5. Resolved at render time only, never baked into
the Standard file — `.standard_sch` stores literal `${TITLE}` strings so the
round-trip stays lossless.

**Rules:**
- Tokens are `${IDENT}` where IDENT matches `[A-Za-z_][A-Za-z0-9_]*`.
- Unknown tokens resolve to empty string (never literal `${FOO}`).
- Strings that look like tokens but aren't (whitespace, punctuation
inside the braces) pass through verbatim — the scanner doesn't match.
- Works in any text object, not just title blocks.

## Relationships

| Type | Target |
|------|--------|
| related | [SubstitutionContext](/crates/oxide-output/src/substitution/SubstitutionContext.md) |
| related | [lookup](/crates/oxide-output/src/substitution/lookup.md) |
| related | [lookup](/crates/oxide-output/src/substitution/lookup.md) |
| related | [resolve](/crates/oxide-output/src/substitution/resolve.md) |
| related | [scan_token](/crates/oxide-output/src/substitution/scan_token.md) |
| related | [is_ident_start](/crates/oxide-output/src/substitution/is_ident_start.md) |
| related | [is_ident_continue](/crates/oxide-output/src/substitution/is_ident_continue.md) |
| related | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| related | [ctx](/crates/oxide-output/src/substitution/ctx.md) |
| related | [resolves_builtin_tokens](/crates/oxide-output/src/substitution/resolves_builtin_tokens.md) |
| related | [resolves_custom_fields](/crates/oxide-output/src/substitution/resolves_custom_fields.md) |
| related | [unknown_token_renders_empty](/crates/oxide-output/src/substitution/unknown_token_renders_empty.md) |
| related | [malformed_passes_through](/crates/oxide-output/src/substitution/malformed_passes_through.md) |
| related | [multiple_tokens_same_line](/crates/oxide-output/src/substitution/multiple_tokens_same_line.md) |
| related | [multiline_text](/crates/oxide-output/src/substitution/multiline_text.md) |
| related | [plain_text_unchanged](/crates/oxide-output/src/substitution/plain_text_unchanged.md) |
| related | [preserves_utf8](/crates/oxide-output/src/substitution/preserves_utf8.md) |
| related | [dollar_without_brace_passes_through](/crates/oxide-output/src/substitution/dollar_without_brace_passes_through.md) |
| related | [variant_token_only_emits_when_physical_structure_on](/crates/oxide-output/src/substitution/variant_token_only_emits_when_physical_structure_on.md) |
| related | [sheet_number_token_drops_when_toggle_off](/crates/oxide-output/src/substitution/sheet_number_token_drops_when_toggle_off.md) |
| related | [document_number_falls_back_to_custom_field](/crates/oxide-output/src/substitution/document_number_falls_back_to_custom_field.md) |
