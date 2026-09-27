---
okf_version: "0.2"
type: Module
title: lib
description: Hierarchical constraint manager and deterministic design rule check (DRC) engine for Oxide EDA.
resource: crates/oxide-rules/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:49Z"
concept_id: crates/oxide-rules/src/lib
language: rust
---

# lib

Hierarchical constraint manager and deterministic design rule check (DRC) engine for Oxide EDA.

## Docstring

Hierarchical constraint manager and deterministic design rule check (DRC) engine for Oxide EDA.

# Core Philosophy
"AI proposes, Deterministic Rules validate."

All geometric constraints, electrical clearances, routing widths, and impedance matching rules
are stored hierarchically and evaluated strictly:
`Net` (highest) > `NetClass` > `Room` > `Global` (lowest).
