---
okf_version: "0.2"
type: Function
title: messages_panel_capture
description: "Make sure the real in-app logger is installed, so"
resource: crates/oxide-app/src/library/resolve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/resolve/messages_panel_capture
language: rust
---

# messages_panel_capture

Make sure the real in-app logger is installed, so

## Signature

```rust
fn messages_panel_capture()
```

## Docstring

Make sure the real in-app logger is installed, so
`recent_entries()` shows what the Messages panel would show.
Other tests in this binary install it too; whoever gets there
first installs the same `OxideLogger` feeding the same sink,
so a rejected second call changes nothing.

## Source
Lines 84–88 in `crates/oxide-app/src/library/resolve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolve](/crates/oxide-app/src/library/resolve.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| called_by | [a_read_failure_reaches_the_messages_panel_before_collapsing_to_none](/crates/oxide-app/src/library/resolve/a_read_failure_reaches_the_messages_panel_before_collapsing_to_none.md) |
| called_by | [an_absent_primitive_stays_none_and_is_not_reported_as_a_failure](/crates/oxide-app/src/library/resolve/an_absent_primitive_stays_none_and_is_not_reported_as_a_failure.md) |
