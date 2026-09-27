---
okf_version: "0.2"
type: Class
title: TemplateId
description: Opaque identifier for a template. Built-ins use well-known strings
resource: crates/oxide-output/src/template/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/mod/TemplateId
language: rust
---

# TemplateId

Opaque identifier for a template. Built-ins use well-known strings

## Signature

```rust
pub struct TemplateId
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Opaque identifier for a template. Built-ins use well-known strings
(`"iso_a4_landscape"`, `"ansi_c_landscape"`); user templates will later
use a project-relative path.
[derive(Debug, Clone, PartialEq, Eq, Hash)]

## Source
Lines 27–27 in `crates/oxide-output/src/template/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [template](/crates/oxide-output/src/template/mod.md) |
