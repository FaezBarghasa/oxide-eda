---
okf_version: "0.2"
type: Function
title: missing_required_for_test
description: Build the validation list for a draft — wraps
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/missing_required_for_test
language: rust
---

# missing_required_for_test

Build the validation list for a draft — wraps

## Signature

```rust
fn missing_required_for_test(
    registry: &oxide_library::TemplateRegistry,
    library_id: uuid::Uuid,
    class: &str,
    params: &oxide_library::ParamMap,
) -> Vec<String>
```

## Decorators

- `cfg(test)`

## Docstring

Build the validation list for a draft — wraps
`TemplateRegistry::validate_params` so view-level tests can assert
against a stable shape regardless of the registry's internal layout.
[cfg(test)]

## Source
Lines 516–531 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| called_by | [template_with_missing_required_flag_visible](/crates/oxide-app/src/library/editor/params/template_with_missing_required_flag_visible.md) |
| called_by | [template_with_required_and_optional_fully_populated](/crates/oxide-app/src/library/editor/params/template_with_required_and_optional_fully_populated.md) |
