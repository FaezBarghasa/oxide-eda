---
okf_version: "0.2"
type: Function
title: resolve
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/resolve
language: rust
---

# resolve

## Signature

```rust
fn resolve(field: &'static str, value: &str, edit: OptionalNumberEdit) -> Option<Option<f64>>
```

## Source
Lines 118–120 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| calls | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
