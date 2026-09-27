---
okf_version: "0.2"
type: Function
title: report_constraint_not_added
description: Report a constraint the user asked for and did not get.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/report_constraint_not_added
language: rust
---

# report_constraint_not_added

Report a constraint the user asked for and did not get.

## Signature

```rust
fn report_constraint_not_added(
    editor: &mut crate::app::FootprintEditorState,
    tag: crate::library::messages::SketchConstraintTag,
    bad_dimension: Option<String>,
)
```

## Docstring

Report a constraint the user asked for and did not get.

GH #599 — the dimensional tags (Distance, Angle, point-to-circle,
point-to-line) drop the `ParseFloatError` from `dimension_input`, and
every tag drops a selection that does not match its arm. Both used to
end in the same silent no-op. Reported at `error!`: the requested
constraint was withheld outright, and the default log filter is
`LevelFilter::Info`, so `debug!` would never reach the Messages panel.

## Source
Lines 272–304 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.md) |
| called_by | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
