---
okf_version: "0.2"
type: Module
title: src
description: Oxide EDA — AI-first electronics design automation.
resource: crates/oxide-app/src/main.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/main
language: rust
---

# src

Oxide EDA — AI-first electronics design automation.

## Docstring

Oxide EDA — AI-first electronics design automation.

Thin binary entrypoint. The real implementation lives in the
library face (`lib.rs`) so integration tests can drive
dispatchers without spinning up the iced runtime.

## Relationships

| Type | Target |
|------|--------|
| related | [main](/crates/oxide-app/src/main/main.md) |
