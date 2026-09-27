---
okf_version: "0.2"
type: Function
title: distributor_label
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/distributor_label
language: rust
---

# distributor_label

## Signature

```rust
fn distributor_label(s: DistributorSource) -> &'static str
```

## Source
Lines 83–93 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| called_by | [distributor_source_to_string](/crates/oxide-app/src/library/editor/supply/distributor_source_to_string.md) |
| called_by | [fmt](/crates/oxide-app/src/library/editor/supply/fmt.md) |
