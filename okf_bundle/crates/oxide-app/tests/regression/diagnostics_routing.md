---
okf_version: "0.2"
type: Module
title: diagnostics_routing
description: Records emitted while handling a message have to be on screen in the
resource: crates/oxide-app/tests/regression/diagnostics_routing.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/diagnostics_routing
language: rust
---

# diagnostics_routing

Records emitted while handling a message have to be on screen in the

## Docstring

Records emitted while handling a message have to be on screen in the
Messages panel by the time that message finishes.

The panel does not read the diagnostics ring buffer directly — it
renders `document_state.panel_ctx.diagnostics`, which is a snapshot
republished by `sync_diagnostics_panel_ctx`. `finish_update` calls
that, but not every dispatcher calls `finish_update`:
`dispatch_preferences_message` deliberately does not, because
reloading the History panel and draining git commits on every
keystroke in a modal is not what that dispatcher is for. Before this
was wired, a Preferences failure sat in the ring buffer until some
unrelated later message happened to republish it — so the log window
showed the failure attached to the wrong action, or not at all.

These drive the real dispatcher through `Oxide::update`, so they fail
if the republish call is removed.

## Relationships

| Type | Target |
|------|--------|
| related | [ensure_logger](/crates/oxide-app/tests/regression/diagnostics_routing/ensure_logger.md) |
| related | [a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot](/crates/oxide-app/tests/regression/diagnostics_routing/a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot.md) |
| related | [the_panel_snapshot_is_refreshed_on_every_preferences_message](/crates/oxide-app/tests/regression/diagnostics_routing/the_panel_snapshot_is_refreshed_on_every_preferences_message.md) |
