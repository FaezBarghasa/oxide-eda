# parse

## Classs

- [Lexer](Lexer.md) — Scans the source string one token at a time on demand.
- [Parser](Parser.md) — Recursive-descent parser. Holds a single token of look-ahead so
- [Spanned](Spanned.md) — A token paired with the byte offset of its first character in the
- [Token](Token.md) — One lexical token.

## Functions

- [bump](bump.md) — Consume the current look-ahead and refill from the lexer.
- [bump](bump_1.md) — Consume the current look-ahead and refill from the lexer.
- [expect](expect.md) — Consume the current token if it equals `expected`, otherwise
- [expect](expect_1.md) — Consume the current token if it equals `expected`, otherwise
- [lex_ident](lex_ident.md) — Eat an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`. The cursor is on
- [lex_ident](lex_ident_1.md) — Eat an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`. The cursor is on
- [lex_quantity](lex_quantity.md) — Eat a quantity literal: digits/dot followed by optional letter
- [lex_quantity](lex_quantity_1.md) — Eat a quantity literal: digits/dot followed by optional letter
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [next_token](next_token.md) — Produce the next token, advancing the cursor.
- [next_token](next_token_1.md) — Produce the next token, advancing the cursor.
- [parse](parse.md) — Parse a source string into an [`ExprNode`] tree.
- [parse_and](parse_and.md)
- [parse_and](parse_and_1.md)
- [parse_comparison](parse_comparison.md)
- [parse_comparison](parse_comparison_1.md)
- [parse_equality](parse_equality.md) — -- equality / comparison ----------------------------------------
- [parse_equality](parse_equality_1.md) — -- equality / comparison ----------------------------------------
- [parse_expr](parse_expr.md) — -- expr ----------------------------------------------------------
- [parse_expr](parse_expr_1.md) — -- expr ----------------------------------------------------------
- [parse_list](parse_list.md) — `list ::= expr (',' expr)*` — parses until the next `]`.
- [parse_list](parse_list_1.md) — `list ::= expr (',' expr)*` — parses until the next `]`.
- [parse_lookup_call](parse_lookup_call.md) — `lookup` has already been consumed and `(` is the look-ahead.
- [parse_lookup_call](parse_lookup_call_1.md) — `lookup` has already been consumed and `(` is the look-ahead.
- [parse_or](parse_or.md) — -- or / and ------------------------------------------------------
- [parse_or](parse_or_1.md) — -- or / and ------------------------------------------------------
- [parse_power](parse_power.md) — Right-associative: `2^3^2` = `2^(3^2)` = 512.
- [parse_power](parse_power_1.md) — Right-associative: `2^3^2` = `2^(3^2)` = 512.
- [parse_primary](parse_primary.md) — -- primary -------------------------------------------------------
- [parse_primary](parse_primary_1.md) — -- primary -------------------------------------------------------
- [parse_product](parse_product.md)
- [parse_product](parse_product_1.md)
- [parse_sum](parse_sum.md) — -- sum / product / power ----------------------------------------
- [parse_sum](parse_sum_1.md) — -- sum / product / power ----------------------------------------
- [parse_ternary](parse_ternary.md) — -- ternary -------------------------------------------------------
- [parse_ternary](parse_ternary_1.md) — -- ternary -------------------------------------------------------
- [parse_unary](parse_unary.md) — -- unary ---------------------------------------------------------
- [parse_unary](parse_unary_1.md) — -- unary ---------------------------------------------------------
- [peek_byte](peek_byte.md) — Look at the byte at the current cursor without consuming it.
- [peek_byte](peek_byte_1.md) — Look at the byte at the current cursor without consuming it.
- [peek_byte_at](peek_byte_at.md) — Look at the byte one past the current cursor.
- [peek_byte_at](peek_byte_at_1.md) — Look at the byte one past the current cursor.
- [skip_ws](skip_ws.md) — Skip ASCII whitespace.
- [skip_ws](skip_ws_1.md) — Skip ASCII whitespace.
- [smoke_addition](smoke_addition.md) — [test]
- [smoke_eof_after_expr](smoke_eof_after_expr.md) — [test]
- [smoke_literal](smoke_literal.md) — [test]
