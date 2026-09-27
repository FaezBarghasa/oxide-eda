---
okf_version: "0.2"
type: Module
title: resolve
description: "Reporting wrapper around [`oxide_library::LibrarySet`] resolution."
resource: crates/oxide-app/src/library/resolve.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/resolve
language: rust
---

# resolve

Reporting wrapper around [`oxide_library::LibrarySet`] resolution.

## Docstring

Reporting wrapper around [`oxide_library::LibrarySet`] resolution.

The resolvers return `Result<Option<T>, LibraryError>`: `Ok(None)`
is "the primitive is not there", `Err` is "the lookup itself
failed". UI state slots (`state.symbol`, `state.footprint`,
`state.sim`) can only hold an `Option`, so the two have to be
flattened somewhere — [`report_read_failure`] is the one place
that is allowed to do it, and it puts the error in front of the
user first.

Without this the user is told a UUID is not in the mounted
libraries when the truth is that a `.snxsym` on disk is corrupt or
a remote library server answered 500 — a diagnosis that sends them
off to re-bind a perfectly good reference.

## Relationships

| Type | Target |
|------|--------|
| related | [ResolvedKind](/crates/oxide-app/src/library/resolve/ResolvedKind.md) |
| related | [label](/crates/oxide-app/src/library/resolve/label.md) |
| related | [label](/crates/oxide-app/src/library/resolve/label.md) |
| related | [report_read_failure](/crates/oxide-app/src/library/resolve/report_read_failure.md) |
| related | [any_ref](/crates/oxide-app/src/library/resolve/any_ref.md) |
| related | [messages_panel_capture](/crates/oxide-app/src/library/resolve/messages_panel_capture.md) |
| related | [entries_mentioning](/crates/oxide-app/src/library/resolve/entries_mentioning.md) |
| related | [a_resolved_primitive_passes_straight_through](/crates/oxide-app/src/library/resolve/a_resolved_primitive_passes_straight_through.md) |
| related | [an_absent_primitive_stays_none_and_is_not_reported_as_a_failure](/crates/oxide-app/src/library/resolve/an_absent_primitive_stays_none_and_is_not_reported_as_a_failure.md) |
| related | [a_read_failure_reaches_the_messages_panel_before_collapsing_to_none](/crates/oxide-app/src/library/resolve/a_read_failure_reaches_the_messages_panel_before_collapsing_to_none.md) |
| related | [every_kind_has_its_own_wording](/crates/oxide-app/src/library/resolve/every_kind_has_its_own_wording.md) |
