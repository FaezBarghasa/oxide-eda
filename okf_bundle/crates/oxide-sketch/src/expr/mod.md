---
okf_version: "0.2"
type: Module
title: expr
description: Expression language for the sketch parameter table.
resource: crates/oxide-sketch/src/expr/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/mod
language: rust
---

# expr

Expression language for the sketch parameter table.

## Docstring

Expression language for the sketch parameter table.

Parameters can carry expressions — `body_w = "= pad_pitch *
(pin_count - 1) + 2.0mm"` — which are parsed into the
[`ast::ExprNode`] tree by [`parse::parse`] and evaluated to a
canonical-unit `f64` by [`eval::eval`]. The evaluator carries
[`unit::Quantity`] values throughout and unit-checks every
binary op so a `mm + deg` expression fails fast.

The persisted on-disk form is the source `String`; the AST is
rebuilt on load by re-parsing.

## Relationships

| Type | Target |
|------|--------|
| related | [ExprError](/crates/oxide-sketch/src/expr/mod/ExprError.md) |
| related | [from](/crates/oxide-sketch/src/expr/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/expr/mod/from.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
