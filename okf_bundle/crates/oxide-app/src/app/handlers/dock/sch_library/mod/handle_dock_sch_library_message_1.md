---
okf_version: "0.2"
type: Function
title: handle_dock_sch_library_message
description: "Returns `None` when the message isn't an SCH-library message (so"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message_1
language: rust
---

# handle_dock_sch_library_message

Returns `None` when the message isn't an SCH-library message (so

## Signature

```rust
pub(super) fn handle_dock_sch_library_message(
        &mut self,
        panel_msg: &PanelMsg,
    ) -> Option<Task<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Returns `None` when the message isn't an SCH-library message (so
the caller falls through to the next dock handler), or `Some(task)`
when handled — the task carries any follow-up work from a
re-entrant `self.update(...)` so it isn't dropped.

## Source
Lines 58–764 in `crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_library](/crates/oxide-app/src/app/handlers/dock/sch_library/mod.md) |
| calls | [fp_parse_optional_mm](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/fp_parse_optional_mm.md) |
| calls | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
| calls | [fp_parse_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number.md) |
| calls | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
| calls | [apply_graphic_field](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/apply_graphic_field.md) |
