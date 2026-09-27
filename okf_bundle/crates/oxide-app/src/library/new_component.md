---
okf_version: "0.2"
type: Module
title: new_component
description: "\"New Component\" modal — opened from File ▸ Library ▸ New"
resource: crates/oxide-app/src/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/new_component
language: rust
---

# new_component

"New Component" modal — opened from File ▸ Library ▸ New

## Docstring

"New Component" modal — opened from File ▸ Library ▸ New
Component… and from the project tree's library-node right-click
menu.

Components are rows in category tables (DBLib model). The modal
collects PN + library + table + class. On submit the dispatcher
calls `commands::create_component_row` which mints Symbol +
Footprint primitives, builds a `ComponentRow` with the binding
refs, and inserts it into the chosen table. Success fires
`LibraryMessage::OpenComponentRow` so the new row opens as a
Component Preview tab.

Shape (plan §13):

```text
┌─[ New Component ]──────────────────────────────┐
│ Internal PN [_______________________________]  │
│ Library     [▾ MyComponents              ]     │
│ Table       [▾ Resistors                 ]     │
│ Class       [▾ resistor                  ]     │
│             [ Cancel ]  [ Create Row ]         │
└────────────────────────────────────────────────┘
```

When the manifest declares no `[[tables]]` overrides we still
surface the table pick_list with a single "<class>s" placeholder
option so the user always sees the destination filename.

## Relationships

| Type | Target |
|------|--------|
| related | [LibraryPick](/crates/oxide-app/src/library/new_component/LibraryPick.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [ClassPick](/crates/oxide-app/src/library/new_component/ClassPick.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [TablePick](/crates/oxide-app/src/library/new_component/TablePick.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/new_component/fmt.md) |
| related | [view](/crates/oxide-app/src/library/new_component/view.md) |
| related | [close_x](/crates/oxide-app/src/library/new_component/close_x.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
