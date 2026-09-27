---
okf_version: "0.2"
type: Function
title: fixture_sim
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/fixture_sim
language: rust
---

# fixture_sim

## Signature

```rust
fn fixture_sim(name: &str) -> SimModel
```

## Source
Lines 191–203 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| called_by | [list_footprints_and_sims_are_name_sorted](/crates/oxide-library/tests/local_git_adapter/list_footprints_and_sims_are_name_sorted.md) |
| called_by | [primitive_saves_each_create_a_commit](/crates/oxide-library/tests/local_git_adapter/primitive_saves_each_create_a_commit.md) |
| called_by | [save_then_get_sim_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_sim_round_trip.md) |
