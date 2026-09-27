---
okf_version: "0.2"
type: Function
title: validate_endpoints
resource: crates/oxide-physics/src/hdi.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/hdi/validate_endpoints
language: rust
---

# validate_endpoints

## Signature

```rust
fn validate_endpoints(
    start: usize,
    end: usize,
    stackup: &LayerStackup,
    total_layers: usize,
) -> Result<(), HdiError>
```

## Source
Lines 236–267 in `crates/oxide-physics/src/hdi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hdi](/crates/oxide-physics/src/hdi.md) |
| called_by | [check_hdi_rules](/crates/oxide-physics/src/hdi/check_hdi_rules.md) |
