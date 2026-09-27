---
okf_version: "0.2"
type: Function
title: resolve
description: "Lookup order (per plan §4.3):"
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates/resolve
language: rust
---

# resolve

Lookup order (per plan §4.3):

## Signature

```rust
impl TemplateRegistry { pub fn resolve(&self, library_id: Uuid, class: &str) -> Option<&ParameterTemplate> }
```

## Visibility

- `pub`

## Docstring

Lookup order (per plan §4.3):
1. `per_lib[(library_id, class)]`,
2. `global[class]`,
3. `None` (no template; validation trivially passes).

## Source
Lines 168–173 in `crates/oxide-library/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/oxide-library/src/templates.md) |
