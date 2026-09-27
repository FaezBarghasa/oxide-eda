---
okf_version: "0.2"
type: Module
title: symbol
description: "Symbol canvas editor messages — the `Symbol*` family split out of the"
resource: crates/oxide-app/src/library/messages/symbol.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/symbol
language: rust
---

# symbol

Symbol canvas editor messages — the `Symbol*` family split out of the

## Docstring

Symbol canvas editor messages — the `Symbol*` family split out of the
former flat `PrimitiveEditorMsg` (ADR-0001 D3). Reached through
[`super::PrimitiveEdit::Symbol`]; matched by `apply_symbol_primitive_edit`.

## Relationships

| Type | Target |
|------|--------|
| related | [SymbolEditorMsg](/crates/oxide-app/src/library/messages/symbol/SymbolEditorMsg.md) |
