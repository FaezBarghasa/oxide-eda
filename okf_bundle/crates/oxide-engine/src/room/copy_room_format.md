---
okf_version: "0.2"
type: Function
title: copy_room_format
description: "Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms."
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/copy_room_format
language: rust
---

# copy_room_format

Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms.

## Signature

```rust
impl RoomManager { pub fn copy_room_format(
        &self,
        board: &mut PcbBoard,
        source_room_id: Uuid,
        target_room_ids: &[Uuid],
        options: RoomCopyOptions,
    ) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms.

This achieves 100% Altium Designer "Copy Room Formats" behavior for multi-channel designs.
When repeating schematic sheets / channels (e.g. Channel 1 -> Channel 2, 3, 4), the relative
offsets from the room origin/anchor are precisely transferred to the corresponding components in target rooms.

## Source
Lines 133–257 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
| calls | [strip_channel_suffix](/crates/oxide-engine/src/room/strip_channel_suffix.md) |
