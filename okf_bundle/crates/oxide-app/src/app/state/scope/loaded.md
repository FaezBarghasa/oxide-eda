---
okf_version: "0.2"
type: Function
title: loaded
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/loaded
language: rust
---

# loaded

## Signature

```rust
fn loaded(entries: &[(&str, &[&str])]) -> HashMap<PathBuf, Vec<String>>
```

## Source
Lines 148–158 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| called_by | [a_hierarchy_cycle_terminates_instead_of_looping](/crates/oxide-app/src/app/state/scope/a_hierarchy_cycle_terminates_instead_of_looping.md) |
| called_by | [grandchild_resolves_through_two_hops](/crates/oxide-app/src/app/state/scope/grandchild_resolves_through_two_hops.md) |
| called_by | [hierarchical_child_resolves_to_the_project_of_its_root](/crates/oxide-app/src/app/state/scope/hierarchical_child_resolves_to_the_project_of_its_root.md) |
| called_by | [shared_child_ownership_is_deterministic_regardless_of_map_order](/crates/oxide-app/src/app/state/scope/shared_child_ownership_is_deterministic_regardless_of_map_order.md) |
| called_by | [sheet_inside_the_project_directory_but_unlisted_belongs_to_no_project](/crates/oxide-app/src/app/state/scope/sheet_inside_the_project_directory_but_unlisted_belongs_to_no_project.md) |
