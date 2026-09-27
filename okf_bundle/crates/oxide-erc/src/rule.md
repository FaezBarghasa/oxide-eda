---
okf_version: "0.2"
type: Module
title: rule
description: "Rule metadata types. Describe *what* a rule checks and *where* it applies."
resource: crates/oxide-erc/src/rule.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/rule
language: rust
---

# rule

Rule metadata types. Describe *what* a rule checks and *where* it applies.

## Docstring

Rule metadata types. Describe *what* a rule checks and *where* it applies.
The built-in rules are registered in [`crate::engine`]; DSL rules will
produce [`RuleDefinition`] values at compile time (Phase 2).

## Relationships

| Type | Target |
|------|--------|
| related | [RuleId](/crates/oxide-erc/src/rule/RuleId.md) |
| related | [builtin](/crates/oxide-erc/src/rule/builtin.md) |
| related | [user](/crates/oxide-erc/src/rule/user.md) |
| related | [as_str](/crates/oxide-erc/src/rule/as_str.md) |
| related | [builtin](/crates/oxide-erc/src/rule/builtin.md) |
| related | [user](/crates/oxide-erc/src/rule/user.md) |
| related | [as_str](/crates/oxide-erc/src/rule/as_str.md) |
| related | [fmt](/crates/oxide-erc/src/rule/fmt.md) |
| related | [fmt](/crates/oxide-erc/src/rule/fmt.md) |
| related | [RuleTarget](/crates/oxide-erc/src/rule/RuleTarget.md) |
| related | [AnalysisScope](/crates/oxide-erc/src/rule/AnalysisScope.md) |
| related | [Applicability](/crates/oxide-erc/src/rule/Applicability.md) |
| related | [RuleDefinition](/crates/oxide-erc/src/rule/RuleDefinition.md) |
| related | [builtin](/crates/oxide-erc/src/rule/builtin.md) |
| related | [builtin](/crates/oxide-erc/src/rule/builtin.md) |
