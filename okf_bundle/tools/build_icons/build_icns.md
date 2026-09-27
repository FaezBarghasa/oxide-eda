---
okf_version: "0.2"
type: Function
title: build_icns
description: Use Pillow to write a .icns containing common Apple sizes.
resource: tools/build_icons.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:build_icons.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/build_icons/build_icns
language: python
---

# build_icns

Use Pillow to write a .icns containing common Apple sizes.

## Signature

```python
def build_icns(pngs: dict[int, bytes], out: Path) -> None
```

## Docstring

Use Pillow to write a .icns containing common Apple sizes.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `pngs` | `dict[int, bytes]` | `—` |

| `out` | `Path` | `—` |

## Returns
`None`

## Source
Lines 74–78 in `tools/build_icons.py`

## Relationships

| Type | Target |
|------|--------|
| related | [build_icons](/tools/build_icons.md) |
| called_by | [main](/tools/build_icons/main.md) |
