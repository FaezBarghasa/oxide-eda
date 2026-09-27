---
okf_version: "0.2"
type: Function
title: unreadable_text_refuses_the_write
description: "#599 — the comma-decimal keyboard case. Refusing the write is the"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/unreadable_text_refuses_the_write
language: rust
---

# unreadable_text_refuses_the_write

#599 — the comma-decimal keyboard case. Refusing the write is the

## Signature

```rust
fn unreadable_text_refuses_the_write()
```

## Decorators

- `test`

## Docstring

#599 — the comma-decimal keyboard case. Refusing the write is the
whole point: `None` here would have overwritten the stored value.
[test]

## Source
Lines 153–167 in `crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numeric_input](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input.md) |
| calls | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
