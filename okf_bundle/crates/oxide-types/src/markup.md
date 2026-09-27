---
okf_version: "0.2"
type: Module
title: markup
description: Oxide schematic-text markup.
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup
language: rust
---

# markup

Oxide schematic-text markup.

## Docstring

Oxide schematic-text markup.

Markdown-extension style — a small subset of standard Markdown plus
Oxide-specific extensions for technical typography:

`**bold**`           — bold span
`*italic*`           — italic span
`~~strike~~`         — strikethrough
`^superscript^`      — superscript (Oxide extension; not GFM)
`~subscript~`        — subscript (Oxide extension; not GFM)
`_~overbar~_`        — overbar (Oxide extension; for active-low signal naming)
`[label](url)`       — link
`\X`                 — literal X (escape any sigil)

Returns a flat `Vec<RichSegment>`. Spans don't nest in this version
(matching the practical needs of schematic labels, component
refdes/value/comments, pin names, and net names — none of which
typically use nested formatting). If nesting becomes useful, the
parser can be upgraded to a span tree without changing the public
enum's variant set.

Auto net names use the format `unnamed-<sheet>:<ref>:<pin>`. This
is the canonical Oxide spelling — it does not match any other
EDA tool's auto-net format.

Expression substitution (`${refdes:...}`, `@{...}`, `CELL()`,
`NET_NAME(...)`) is preserved from the previous module — it is
Altium-flavoured and was already independent of the Standard markup
syntax.

## Relationships

| Type | Target |
|------|--------|
| related | [RichSegment](/crates/oxide-types/src/markup/RichSegment.md) |
| related | [ExpressionEvalContext](/crates/oxide-types/src/markup/ExpressionEvalContext.md) |
| related | [auto_net_name](/crates/oxide-types/src/markup/auto_net_name.md) |
| related | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
| related | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
| related | [flush_normal](/crates/oxide-types/src/markup/flush_normal.md) |
| related | [read_paired_single](/crates/oxide-types/src/markup/read_paired_single.md) |
| related | [read_paired_double](/crates/oxide-types/src/markup/read_paired_double.md) |
| related | [read_overbar](/crates/oxide-types/src/markup/read_overbar.md) |
| related | [read_paired_bracket](/crates/oxide-types/src/markup/read_paired_bracket.md) |
| related | [read_paired_paren](/crates/oxide-types/src/markup/read_paired_paren.md) |
| related | [unescape](/crates/oxide-types/src/markup/unescape.md) |
| related | [read_braced](/crates/oxide-types/src/markup/read_braced.md) |
| related | [read_parenthesized](/crates/oxide-types/src/markup/read_parenthesized.md) |
| related | [starts_with_ascii_ci](/crates/oxide-types/src/markup/starts_with_ascii_ci.md) |
| related | [lookup_ci](/crates/oxide-types/src/markup/lookup_ci.md) |
| related | [eval_dollar_expression](/crates/oxide-types/src/markup/eval_dollar_expression.md) |
| related | [eval_at_expression](/crates/oxide-types/src/markup/eval_at_expression.md) |
| related | [eval_net_name](/crates/oxide-types/src/markup/eval_net_name.md) |
| related | [plain_text](/crates/oxide-types/src/markup/plain_text.md) |
| related | [empty_input](/crates/oxide-types/src/markup/empty_input.md) |
| related | [bold](/crates/oxide-types/src/markup/bold.md) |
| related | [italic](/crates/oxide-types/src/markup/italic.md) |
| related | [strike](/crates/oxide-types/src/markup/strike.md) |
| related | [superscript](/crates/oxide-types/src/markup/superscript.md) |
| related | [subscript](/crates/oxide-types/src/markup/subscript.md) |
| related | [overbar](/crates/oxide-types/src/markup/overbar.md) |
| related | [link](/crates/oxide-types/src/markup/link.md) |
| related | [escape_sigils](/crates/oxide-types/src/markup/escape_sigils.md) |
| related | [mixed_overbar_and_subscript](/crates/oxide-types/src/markup/mixed_overbar_and_subscript.md) |
| related | [unmatched_sigil_is_literal](/crates/oxide-types/src/markup/unmatched_sigil_is_literal.md) |
| related | [auto_net_name_format_is_oxide](/crates/oxide-types/src/markup/auto_net_name_format_is_oxide.md) |
| related | [auto_net_name_empty_pins](/crates/oxide-types/src/markup/auto_net_name_empty_pins.md) |
| related | [evaluates_refdes_and_at_variables](/crates/oxide-types/src/markup/evaluates_refdes_and_at_variables.md) |
| related | [evaluates_cell_and_net_name](/crates/oxide-types/src/markup/evaluates_cell_and_net_name.md) |
| related | [unresolved_expressions_are_preserved](/crates/oxide-types/src/markup/unresolved_expressions_are_preserved.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
