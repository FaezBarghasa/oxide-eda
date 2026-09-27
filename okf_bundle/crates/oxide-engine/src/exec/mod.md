---
okf_version: "0.2"
type: Module
title: exec
description: "`Engine::execute` command handlers, grouped by command family."
resource: crates/oxide-engine/src/exec/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-engine/src/exec/mod
language: rust
---

# exec

`Engine::execute` command handlers, grouped by command family.

## Docstring

`Engine::execute` command handlers, grouped by command family.

The public `execute` in the crate root is a thin dispatcher: it clones
the pre-image once and routes each `Command` to one of these grouped
handlers. Splitting the giant match keeps every file under the
line-count cap while preserving the exact arm-to-body mapping.
