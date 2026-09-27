---
okf_version: "0.2"
type: Class
title: TemplateRegistry
description: Per-library + global template registry.
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates/TemplateRegistry
language: rust
---

# TemplateRegistry

Per-library + global template registry.

## Signature

```rust
pub struct TemplateRegistry
```

## Decorators

- `derive(Clone, Debug, Default)`

## Visibility

- `pub`

## Docstring

Per-library + global template registry.

`global` keeps one entry per class — populated by `new_with_builtins` and
optionally augmented by `load_global_dir`.
`per_lib` overrides per `(library_id, class)` pair — populated by
`LibrarySet` when a library exposes its own `templates/<class>.toml`.
[derive(Clone, Debug, Default)]

## Methods

- `global`
- `per_lib`

## Source
Lines 117–120 in `crates/oxide-library/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/oxide-library/src/templates.md) |
