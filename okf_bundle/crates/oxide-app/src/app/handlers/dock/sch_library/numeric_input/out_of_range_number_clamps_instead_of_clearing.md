---
okf_version: "0.2"
type: Function
title: out_of_range_number_clamps_instead_of_clearing
description: "#599 — `60` used to erase the stored 25 and drop the key from the"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/out_of_range_number_clamps_instead_of_clearing
language: rust
---

# out_of_range_number_clamps_instead_of_clearing

#599 — `60` used to erase the stored 25 and drop the key from the

## Signature

```rust
fn out_of_range_number_clamps_instead_of_clearing()
```

## Decorators

- `test`

## Docstring

#599 — `60` used to erase the stored 25 and drop the key from the
saved `.snxfpt`. It must land on the nearest expressible value.
[test]

## Source
Lines 138–148 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| calls | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
