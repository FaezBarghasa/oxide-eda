# src

## Subdirectories

- [array](array/index.md)
- [body3d](body3d/index.md)
- [courtyard](courtyard/index.md)
- [cutout](cutout/index.md)
- [ipc7351](ipc7351/index.md)
- [keepout](keepout/index.md)
- [mask](mask/index.md)
- [pad](pad/index.md)
- [parametric](parametric/index.md)
- [pour](pour/index.md)
- [profile](profile/index.md)
- [silk](silk/index.md)
- [vscore](vscore/index.md)

## Modules

- [body3d](body3d.md) — 3D-extrude bake — closed profile on a `PlaneKind::BodyTop` plane
- [courtyard](courtyard.md) — Courtyard bake — turns CourtyardAttr-tagged closed-profile sketches
- [cutout](cutout.md) — Board cutout bake — turns BoardCutoutAttr-tagged closed profiles
- [keepout](keepout.md) — Keepout bake — turns KeepoutAttr-tagged closed profiles into
- [lib](lib.md) — Sketch → footprint primitive bake pipeline.
- [mask](mask.md) — Mask + paste-aperture bake — turns MaskOpeningAttr,
- [pad](pad.md) — Pad bake — turns SketchData + solved state into Vec<Pad>.
- [parametric](parametric.md)
- [pour](pour.md) — Pour bake — turns PourAttr-tagged closed sketch profiles into
- [profile](profile.md) — Closed-profile walker — given a starting Line entity, trace a
- [silk](silk.md) — Silkscreen bake — turns SilkAttr-tagged sketch entities into
- [vscore](vscore.md) — V-score bake — turns VScoreHintAttr-tagged Line entities into
