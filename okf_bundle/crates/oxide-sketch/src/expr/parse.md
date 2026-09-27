---
okf_version: "0.2"
type: Module
title: parse
description: Recursive-descent parser for the sketch expression language.
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse
language: rust
---

# parse

Recursive-descent parser for the sketch expression language.

## Docstring

Recursive-descent parser for the sketch expression language.
Cleanroom — see crate-level doc comment in `expr/mod.rs`.

Reference: standard recursive-descent design per Aho/Sethi/Ullman,
*Compilers: Principles, Techniques, and Tools* (Dragon Book).
Implementation derived from first principles; no third-party
parser source consulted.

# Grammar (left-associative unless noted)

```text
expr        ::= ternary
ternary     ::= or ('?' expr ':' expr)?
or          ::= and ('||' and)*
and         ::= equality ('&&' equality)*
equality    ::= comparison (('==' | '!=') comparison)*
comparison  ::= sum (('<' | '<=' | '>' | '>=') sum)*
sum         ::= product (('+' | '-') product)*
product     ::= power (('*' | '/' | '%') power)*
power       ::= unary ('^' unary)?       // RIGHT-associative
unary       ::= ('-' | '!')? primary
primary     ::= QUANTITY | IDENT | '(' expr ')' | call | array_idx
call        ::= 'lookup' '(' expr ',' '[' list ']' ',' '[' list ']' ')'
array_idx   ::= 'i' | 'j'                // bare 1-char identifiers only
list        ::= expr (',' expr)*
```

# Lexing notes

- Whitespace between tokens is silently skipped.
- A `QUANTITY` lexes greedily as `[0-9.]+[a-zA-Z]*` so suffixes
like `mm`, `mil`, `deg` ride along with the digit run; the
resulting slice is fed to [`crate::unit::parse_quantity`].
- Identifiers are `[a-zA-Z_][a-zA-Z0-9_]*`. The reserved word
`lookup` becomes a function call; bare 1-char `i`/`j` become
[`ArrayIndex::I`] / [`ArrayIndex::J`]; everything else is a
[`ExprNode::Ref`].
- There are no string or boolean literals — comparison and
logical operators yield dimensionless 0/1 at evaluation time.
- There are no comments inside expressions; the source is a
single line.

## Relationships

| Type | Target |
|------|--------|
| related | [Token](/crates/oxide-sketch/src/expr/parse/Token.md) |
| related | [Spanned](/crates/oxide-sketch/src/expr/parse/Spanned.md) |
| related | [Lexer](/crates/oxide-sketch/src/expr/parse/Lexer.md) |
| related | [new](/crates/oxide-sketch/src/expr/parse/new.md) |
| related | [skip_ws](/crates/oxide-sketch/src/expr/parse/skip_ws.md) |
| related | [peek_byte](/crates/oxide-sketch/src/expr/parse/peek_byte.md) |
| related | [peek_byte_at](/crates/oxide-sketch/src/expr/parse/peek_byte_at.md) |
| related | [next_token](/crates/oxide-sketch/src/expr/parse/next_token.md) |
| related | [lex_quantity](/crates/oxide-sketch/src/expr/parse/lex_quantity.md) |
| related | [lex_ident](/crates/oxide-sketch/src/expr/parse/lex_ident.md) |
| related | [new](/crates/oxide-sketch/src/expr/parse/new.md) |
| related | [skip_ws](/crates/oxide-sketch/src/expr/parse/skip_ws.md) |
| related | [peek_byte](/crates/oxide-sketch/src/expr/parse/peek_byte.md) |
| related | [peek_byte_at](/crates/oxide-sketch/src/expr/parse/peek_byte_at.md) |
| related | [next_token](/crates/oxide-sketch/src/expr/parse/next_token.md) |
| related | [lex_quantity](/crates/oxide-sketch/src/expr/parse/lex_quantity.md) |
| related | [lex_ident](/crates/oxide-sketch/src/expr/parse/lex_ident.md) |
| related | [Parser](/crates/oxide-sketch/src/expr/parse/Parser.md) |
| related | [new](/crates/oxide-sketch/src/expr/parse/new.md) |
| related | [bump](/crates/oxide-sketch/src/expr/parse/bump.md) |
| related | [expect](/crates/oxide-sketch/src/expr/parse/expect.md) |
| related | [parse_expr](/crates/oxide-sketch/src/expr/parse/parse_expr.md) |
| related | [parse_ternary](/crates/oxide-sketch/src/expr/parse/parse_ternary.md) |
| related | [parse_or](/crates/oxide-sketch/src/expr/parse/parse_or.md) |
| related | [parse_and](/crates/oxide-sketch/src/expr/parse/parse_and.md) |
| related | [parse_equality](/crates/oxide-sketch/src/expr/parse/parse_equality.md) |
| related | [parse_comparison](/crates/oxide-sketch/src/expr/parse/parse_comparison.md) |
| related | [parse_sum](/crates/oxide-sketch/src/expr/parse/parse_sum.md) |
| related | [parse_product](/crates/oxide-sketch/src/expr/parse/parse_product.md) |
| related | [parse_power](/crates/oxide-sketch/src/expr/parse/parse_power.md) |
| related | [parse_unary](/crates/oxide-sketch/src/expr/parse/parse_unary.md) |
| related | [parse_primary](/crates/oxide-sketch/src/expr/parse/parse_primary.md) |
| related | [parse_lookup_call](/crates/oxide-sketch/src/expr/parse/parse_lookup_call.md) |
| related | [parse_list](/crates/oxide-sketch/src/expr/parse/parse_list.md) |
| related | [new](/crates/oxide-sketch/src/expr/parse/new.md) |
| related | [bump](/crates/oxide-sketch/src/expr/parse/bump.md) |
| related | [expect](/crates/oxide-sketch/src/expr/parse/expect.md) |
| related | [parse_expr](/crates/oxide-sketch/src/expr/parse/parse_expr.md) |
| related | [parse_ternary](/crates/oxide-sketch/src/expr/parse/parse_ternary.md) |
| related | [parse_or](/crates/oxide-sketch/src/expr/parse/parse_or.md) |
| related | [parse_and](/crates/oxide-sketch/src/expr/parse/parse_and.md) |
| related | [parse_equality](/crates/oxide-sketch/src/expr/parse/parse_equality.md) |
| related | [parse_comparison](/crates/oxide-sketch/src/expr/parse/parse_comparison.md) |
| related | [parse_sum](/crates/oxide-sketch/src/expr/parse/parse_sum.md) |
| related | [parse_product](/crates/oxide-sketch/src/expr/parse/parse_product.md) |
| related | [parse_power](/crates/oxide-sketch/src/expr/parse/parse_power.md) |
| related | [parse_unary](/crates/oxide-sketch/src/expr/parse/parse_unary.md) |
| related | [parse_primary](/crates/oxide-sketch/src/expr/parse/parse_primary.md) |
| related | [parse_lookup_call](/crates/oxide-sketch/src/expr/parse/parse_lookup_call.md) |
| related | [parse_list](/crates/oxide-sketch/src/expr/parse/parse_list.md) |
| related | [parse](/crates/oxide-sketch/src/expr/parse/parse.md) |
| related | [smoke_literal](/crates/oxide-sketch/src/expr/parse/smoke_literal.md) |
| related | [smoke_addition](/crates/oxide-sketch/src/expr/parse/smoke_addition.md) |
| related | [smoke_eof_after_expr](/crates/oxide-sketch/src/expr/parse/smoke_eof_after_expr.md) |
