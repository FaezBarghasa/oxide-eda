---
okf_version: "0.2"
type: Function
title: finish_schematic_mutation
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/finish_schematic_mutation_1
language: rust
---

# finish_schematic_mutation

## Signature

```rust
fn finish_schematic_mutation(
        &mut self,
        invalidation: crate::schematic_runtime::RenderInvalidation,
        clear_overlay_cache: bool,
        update_selection_info: bool,
    ) -> bool
```

## Source
Lines 180–225 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
