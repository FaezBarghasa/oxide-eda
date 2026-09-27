---
okf_version: "0.2"
type: Module
title: foundation_round_trip
description: Round-trip integration smoke tests for the v0.9-refactor-2 row model.
resource: crates/oxide-library/tests/foundation_round_trip.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/foundation_round_trip
language: rust
---

# foundation_round_trip

Round-trip integration smoke tests for the v0.9-refactor-2 row model.

## Docstring

Round-trip integration smoke tests for the v0.9-refactor-2 row model.

Per `v0.9-refactor-2-plan.md` §3, the on-disk format flips from
"one .snxprt per component" to "one row inside `tables/<name>.tsv`".
This test verifies the table TSV round-trip and the parameter-template
validation pipeline still works on the new row payload.

## Relationships

| Type | Target |
|------|--------|
| related | [full_row_round_trip_via_tsv](/crates/oxide-library/tests/foundation_round_trip/full_row_round_trip_via_tsv.md) |
| related | [template_registry_validates_round_trip](/crates/oxide-library/tests/foundation_round_trip/template_registry_validates_round_trip.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
