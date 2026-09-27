---
okf_version: "0.2"
type: Module
title: diagnostic
description: "[`Diagnostic`] is the internal output type for every rule. It carries a"
resource: crates/oxide-erc/src/diagnostic.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/diagnostic
language: rust
---

# diagnostic

[`Diagnostic`] is the internal output type for every rule. It carries a

## Docstring

[`Diagnostic`] is the internal output type for every rule. It carries a
[`RuleId`] in addition to the legacy [`RuleKind`] so that DSL rules
(Phase 2) can emit violations without needing a `RuleKind` variant.

[`Violation`] is kept as the public API type; [`From<Diagnostic>`] converts
for backward compatibility with the rest of the app.

## Relationships

| Type | Target |
|------|--------|
| related | [Diagnostic](/crates/oxide-erc/src/diagnostic/Diagnostic.md) |
| related | [new](/crates/oxide-erc/src/diagnostic/new.md) |
| related | [with_severity](/crates/oxide-erc/src/diagnostic/with_severity.md) |
| related | [with_primary](/crates/oxide-erc/src/diagnostic/with_primary.md) |
| related | [with_peer](/crates/oxide-erc/src/diagnostic/with_peer.md) |
| related | [new](/crates/oxide-erc/src/diagnostic/new.md) |
| related | [with_severity](/crates/oxide-erc/src/diagnostic/with_severity.md) |
| related | [with_primary](/crates/oxide-erc/src/diagnostic/with_primary.md) |
| related | [with_peer](/crates/oxide-erc/src/diagnostic/with_peer.md) |
| related | [from](/crates/oxide-erc/src/diagnostic/from.md) |
| related | [from](/crates/oxide-erc/src/diagnostic/from.md) |
