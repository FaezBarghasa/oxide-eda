# markup

## Classs

- [ExpressionEvalContext](ExpressionEvalContext.md) — [derive(Debug, Clone, Default)]
- [RichSegment](RichSegment.md) — [derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Functions

- [auto_net_name](auto_net_name.md) — Default name for an unnamed net.
- [auto_net_name_empty_pins](auto_net_name_empty_pins.md) — [test]
- [auto_net_name_format_is_oxide](auto_net_name_format_is_oxide.md) — [test]
- [bold](bold.md) — [test]
- [empty_input](empty_input.md) — [test]
- [escape_sigils](escape_sigils.md) — [test]
- [eval_at_expression](eval_at_expression.md)
- [eval_dollar_expression](eval_dollar_expression.md)
- [eval_net_name](eval_net_name.md)
- [evaluate_expressions](evaluate_expressions.md) — Evaluate a subset of Altium-style expression variables.
- [evaluates_cell_and_net_name](evaluates_cell_and_net_name.md) — [test]
- [evaluates_refdes_and_at_variables](evaluates_refdes_and_at_variables.md) — [test]
- [flush_normal](flush_normal.md)
- [italic](italic.md) — [test]
- [link](link.md) — [test]
- [lookup_ci](lookup_ci.md)
- [mixed_overbar_and_subscript](mixed_overbar_and_subscript.md) — [test]
- [overbar](overbar.md) — [test]
- [parse_oxide_markup](parse_oxide_markup.md) — Parse Oxide markup into a flat list of rich segments.
- [plain_text](plain_text.md) — [test]
- [read_braced](read_braced.md)
- [read_overbar](read_overbar.md) — Read the body of an overbar `_~ ... ~_` (already past the opening `_~`).
- [read_paired_bracket](read_paired_bracket.md)
- [read_paired_double](read_paired_double.md) — Find a doubled sigil (e.g. `**` or `~~`) and capture the content between.
- [read_paired_paren](read_paired_paren.md)
- [read_paired_single](read_paired_single.md) — Find the next single sigil byte and capture the content between.
- [read_parenthesized](read_parenthesized.md)
- [starts_with_ascii_ci](starts_with_ascii_ci.md)
- [strike](strike.md) — [test]
- [subscript](subscript.md) — [test]
- [superscript](superscript.md) — [test]
- [unescape](unescape.md)
- [unmatched_sigil_is_literal](unmatched_sigil_is_literal.md) — [test]
- [unresolved_expressions_are_preserved](unresolved_expressions_are_preserved.md) — [test]
