---
okf_version: "0.2"
type: Module
title: parser
description: "Chumsky 0.12 parser: DSL source text → `Vec<RuleAst>`."
resource: crates/oxide-erc-dsl/src/parser.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc-dsl/src/parser
language: rust
---

# parser

Chumsky 0.12 parser: DSL source text → `Vec<RuleAst>`.

## Docstring

Chumsky 0.12 parser: DSL source text → `Vec<RuleAst>`.

Grammar (whitespace-insensitive between tokens):

```text
program   := rule_decl*

rule_decl := "rule" IDENT
("apply_to" applicability)?
"on"    target
("scope" scope)?
"when"  expr
"then"  severity STRING

applicability := "sheets" "tagged" STRING
|  "sheets" "named"  STRING

target    := "net" | "pin" | "component" | "sheet"
scope     := "local" | "sheet" | "hierarchical" | "global"
severity  := "error" | "warning" | "info"

expr      := or_expr
or_expr   := and_expr  ("or"  and_expr)*
and_expr  := not_expr  ("and" not_expr)*
not_expr  := "not" not_expr | primary

primary   := "(" expr ")"
|  IDENT "." IDENT "(" args? ")" cmp_or_matches
|  IDENT "." IDENT              cmp_or_matches
|  IDENT "(" args? ")"

cmp_or_matches := ("==" | "!=") literal
|  "matches" STRING

args      := literal ("," literal)*
literal   := STRING | "true" | "false" | IDENT
```

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-erc-dsl/src/parser/parse.md) |
| related | [program_parser](/crates/oxide-erc-dsl/src/parser/program_parser.md) |
| related | [rule_parser](/crates/oxide-erc-dsl/src/parser/rule_parser.md) |
| related | [expr_parser](/crates/oxide-erc-dsl/src/parser/expr_parser.md) |
| related | [primary_parser](/crates/oxide-erc-dsl/src/parser/primary_parser.md) |
| related | [string_lit_parser](/crates/oxide-erc-dsl/src/parser/string_lit_parser.md) |
| related | [literal_parser](/crates/oxide-erc-dsl/src/parser/literal_parser.md) |
| related | [FieldRhs](/crates/oxide-erc-dsl/src/parser/FieldRhs.md) |
