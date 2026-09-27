---
okf_version: "0.2"
type: Function
title: validate_params
description: Validate a parameter map against its class template (looked up via
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates/validate_params
language: rust
---

# validate_params

Validate a parameter map against its class template (looked up via

## Signature

```rust
impl TemplateRegistry { pub fn validate_params(
        &self,
        library_id: Uuid,
        class: &str,
        params: &ParamMap,
    ) -> Vec<TemplateViolation> }
```

## Visibility

- `pub`

## Docstring

Validate a parameter map against its class template (looked up via
`library_id` + `class`). Empty result = pass.

## Source
Lines 177–202 in `crates/oxide-library/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/oxide-library/src/templates.md) |
| calls | [check_slot](/crates/oxide-library/src/templates/check_slot.md) |
