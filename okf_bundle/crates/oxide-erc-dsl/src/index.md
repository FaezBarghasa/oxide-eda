# src

## Subdirectories

- [ast](ast/index.md)
- [compiler](compiler/index.md)
- [error](error/index.md)
- [lib](lib/index.md)
- [parser](parser/index.md)
- [validator](validator/index.md)

## Modules

- [ast](ast.md) — Abstract syntax tree produced by the DSL parser.
- [compiler](compiler.md) — Rule compiler: `RuleAst` -> `CompiledRule` with pre-compiled regex patterns.
- [error](error.md) — Error types for the ERC DSL pipeline.
- [lib](lib.md) — ERC DSL parser and validator.
- [parser](parser.md) — Chumsky 0.12 parser: DSL source text → `Vec<RuleAst>`.
- [validator](validator.md) — AST validator: checks predicate whitelist and target compatibility.
