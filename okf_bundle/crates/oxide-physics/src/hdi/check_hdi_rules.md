---
okf_version: "0.2"
type: Function
title: check_hdi_rules
description: Verify that a via structure obeys physical manufacturing and stackup constraints.
resource: crates/oxide-physics/src/hdi.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/hdi/check_hdi_rules
language: rust
---

# check_hdi_rules

Verify that a via structure obeys physical manufacturing and stackup constraints.

## Signature

```rust
pub fn check_hdi_rules(via: &ViaDefinition, stackup: &LayerStackup) -> Result<(), HdiError>
```

## Visibility

- `pub`

## Docstring

Verify that a via structure obeys physical manufacturing and stackup constraints.

## Source
Lines 129–234 in `crates/oxide-physics/src/hdi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hdi](/crates/oxide-physics/src/hdi.md) |
| calls | [validate_endpoints](/crates/oxide-physics/src/hdi/validate_endpoints.md) |
