---
okf_version: "0.2"
type: Function
title: check_slot
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates/check_slot
language: rust
---

# check_slot

## Signature

```rust
fn check_slot(slot: &ParamSlot, v: &ParamValue, out: &mut Vec<TemplateViolation>)
```

## Source
Lines 214–233 in `crates/oxide-library/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/oxide-library/src/templates.md) |
| calls | [kind_of](/crates/oxide-library/src/templates/kind_of.md) |
| called_by | [validate_params](/crates/oxide-library/src/templates/validate_params.md) |
