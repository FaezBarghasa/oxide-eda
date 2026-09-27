---
okf_version: "0.2"
type: Module
title: test_support
description: "Fixtures shared by this crate's in-src `#[cfg(test)]` modules."
resource: crates/oxide-engine/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/test_support
language: rust
---

# test_support

Fixtures shared by this crate's in-src `#[cfg(test)]` modules.

## Docstring

Fixtures shared by this crate's in-src `#[cfg(test)]` modules.

`SchematicSheet` is an 18-field struct with no `Default`, so every test
module that needs an empty sheet would otherwise carry its own copy of
the literal — and each copy would have to be updated by hand the next
time the sheet grows a field. Mirrors the `test_support` module
`oxide-app` keeps for the same reason.

## Relationships

| Type | Target |
|------|--------|
| related | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
