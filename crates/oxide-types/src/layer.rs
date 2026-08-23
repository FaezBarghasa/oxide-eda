//! Oxide-native PCB layer abstraction.
//!
//! Variants are **semantic** — they describe a layer's purpose
//! (top copper, bottom silkscreen, courtyard, etc.), not its bit
//! position in any particular EDA tool's internal layer set.
//! The previous version of this module exposed `LayerId(u8)` plus
//! pre-Standard-7 numeric constants (`F_CU = 0`, `B_CU = 31`, …) that
//! mirrored Standard's `PCB_LAYER_ID` numbering; those have been
//! removed as part of the issue #62 Apache-clean remediation.
//! Concrete `u8` IDs for any future foreign-format I/O are produced
//! by the `oxide-standard-import` companion crate's translation layer
//! and do not live in this Apache codebase.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// OxideLayer — semantic variant set
// ---------------------------------------------------------------------------

/// A PCB layer identified by purpose, not by index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OxideLayer {
    TopCopper,
    BottomCopper,
    /// 1-based index of an inner-stackup copper layer.
    InnerCopper(u8),
    TopSilk,
    BottomSilk,
    TopSolderMask,
    BottomSolderMask,
    TopPaste,
    BottomPaste,
    TopAssembly,
    BottomAssembly,
    TopCourtyard,
    BottomCourtyard,
    BoardOutline,
    KeepOut,
    /// 1-based index of a user-defined mechanical layer.
    Mechanical(u8),
    /// 1-based index of a generic user layer (notes, comments, etc.).
    User(u8),
}

// ---------------------------------------------------------------------------
// LayerKind — coarse category for theme rendering and picker grouping
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Copper,
    Silk,
    Mask,
    Paste,
    Assembly,
    Courtyard,
    Outline,
    KeepOut,
    Mechanical,
    User,
}

// ---------------------------------------------------------------------------
// OxideLayer methods
// ---------------------------------------------------------------------------

impl OxideLayer {
    pub fn kind(self) -> LayerKind {
        match self {
            Self::TopCopper | Self::BottomCopper | Self::InnerCopper(_) => LayerKind::Copper,
            Self::TopSilk | Self::BottomSilk => LayerKind::Silk,
            Self::TopSolderMask | Self::BottomSolderMask => LayerKind::Mask,
            Self::TopPaste | Self::BottomPaste => LayerKind::Paste,
            Self::TopAssembly | Self::BottomAssembly => LayerKind::Assembly,
            Self::TopCourtyard | Self::BottomCourtyard => LayerKind::Courtyard,
            Self::BoardOutline => LayerKind::Outline,
            Self::KeepOut => LayerKind::KeepOut,
            Self::Mechanical(_) => LayerKind::Mechanical,
            Self::User(_) => LayerKind::User,
        }
    }

    /// Display label for the Oxide UI per `docs/UX_REFERENCE_ALTIUM.md`
    /// and `reference_altium_layer_naming` memory note.
    pub fn altium_label(self) -> String {
        match self {
            Self::TopCopper => "Top Layer".into(),
            Self::BottomCopper => "Bottom Layer".into(),
            Self::InnerCopper(n) => format!("Mid Layer {n}"),
            Self::TopSilk => "Top Overlay".into(),
            Self::BottomSilk => "Bottom Overlay".into(),
            Self::TopSolderMask => "Top Solder".into(),
            Self::BottomSolderMask => "Bottom Solder".into(),
            Self::TopPaste => "Top Paste".into(),
            Self::BottomPaste => "Bottom Paste".into(),
            Self::TopAssembly => "Top Assembly".into(),
            Self::BottomAssembly => "Bottom Assembly".into(),
            Self::TopCourtyard => "Top Courtyard".into(),
            Self::BottomCourtyard => "Bottom Courtyard".into(),
            Self::BoardOutline => "Board Outline".into(),
            Self::KeepOut => "Keep-Out".into(),
            Self::Mechanical(n) => format!("Mechanical {n}"),
            Self::User(n) => format!("User {n}"),
        }
    }

    /// Iterate the canonical fixed-set layers in stable display order.
    /// Excludes the parameterised variants (`InnerCopper`, `Mechanical`,
    /// `User`); callers iterating those provide their own indices.
    pub fn all() -> impl Iterator<Item = OxideLayer> {
        [
            Self::TopCopper,
            Self::BottomCopper,
            Self::TopSilk,
            Self::BottomSilk,
            Self::TopSolderMask,
            Self::BottomSolderMask,
            Self::TopPaste,
            Self::BottomPaste,
            Self::TopAssembly,
            Self::BottomAssembly,
            Self::TopCourtyard,
            Self::BottomCourtyard,
            Self::BoardOutline,
            Self::KeepOut,
        ]
        .into_iter()
    }
}

// ---------------------------------------------------------------------------
// Default layer colours (Altium-flavoured palette, RGBA)
// ---------------------------------------------------------------------------

pub const DEFAULT_LAYER_COLORS: &[(OxideLayer, [u8; 4])] = &[
    (OxideLayer::TopCopper, [0xC8, 0x00, 0x00, 0xFF]), // red
    (OxideLayer::BottomCopper, [0x00, 0x00, 0xC8, 0xFF]), // blue
    (OxideLayer::TopSilk, [0xC8, 0xC8, 0x00, 0xFF]),   // yellow
    (OxideLayer::BottomSilk, [0x80, 0x00, 0x80, 0xFF]), // purple
    (OxideLayer::TopSolderMask, [0xC8, 0x00, 0xC8, 0x80]), // magenta semi
    (OxideLayer::BottomSolderMask, [0x00, 0xC8, 0xC8, 0x80]), // cyan semi
    (OxideLayer::TopPaste, [0x80, 0x80, 0x00, 0xC0]),  // dark yellow
    (OxideLayer::BottomPaste, [0x00, 0x80, 0x80, 0xC0]), // teal
    (OxideLayer::TopAssembly, [0x80, 0x80, 0x80, 0xFF]), // grey
    (OxideLayer::BottomAssembly, [0x60, 0x60, 0x60, 0xFF]), // dark grey
    (OxideLayer::TopCourtyard, [0xC0, 0xC0, 0xC0, 0xFF]), // light grey
    (OxideLayer::BottomCourtyard, [0xA0, 0xA0, 0xA0, 0xFF]), // mid grey
    (OxideLayer::BoardOutline, [0xFF, 0xFF, 0x00, 0xFF]), // bright yellow
    (OxideLayer::KeepOut, [0xFF, 0x00, 0xFF, 0xFF]),   // bright magenta
];

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn altium_labels_match_reference() {
        assert_eq!(OxideLayer::TopCopper.altium_label(), "Top Layer");
        assert_eq!(OxideLayer::BottomCopper.altium_label(), "Bottom Layer");
        assert_eq!(OxideLayer::TopSilk.altium_label(), "Top Overlay");
        assert_eq!(OxideLayer::TopSolderMask.altium_label(), "Top Solder");
        assert_eq!(OxideLayer::TopPaste.altium_label(), "Top Paste");
        assert_eq!(OxideLayer::KeepOut.altium_label(), "Keep-Out");
        assert_eq!(OxideLayer::InnerCopper(2).altium_label(), "Mid Layer 2");
        assert_eq!(OxideLayer::Mechanical(13).altium_label(), "Mechanical 13");
    }

    #[test]
    fn kinds_partition_correctly() {
        assert_eq!(OxideLayer::TopCopper.kind(), LayerKind::Copper);
        assert_eq!(OxideLayer::InnerCopper(1).kind(), LayerKind::Copper);
        assert_eq!(OxideLayer::TopSilk.kind(), LayerKind::Silk);
        assert_eq!(OxideLayer::TopSolderMask.kind(), LayerKind::Mask);
        assert_eq!(OxideLayer::BoardOutline.kind(), LayerKind::Outline);
        assert_eq!(OxideLayer::KeepOut.kind(), LayerKind::KeepOut);
    }

    #[test]
    fn round_trip_json() {
        for l in [
            OxideLayer::TopCopper,
            OxideLayer::InnerCopper(3),
            OxideLayer::Mechanical(4),
            OxideLayer::User(7),
        ] {
            let s = serde_json::to_string(&l).unwrap();
            let back: OxideLayer = serde_json::from_str(&s).unwrap();
            assert_eq!(l, back);
        }
    }

    #[test]
    fn all_iteration_yields_canonical_set() {
        let v: Vec<_> = OxideLayer::all().collect();
        assert_eq!(v.len(), 14);
        assert_eq!(v[0], OxideLayer::TopCopper);
        assert_eq!(v[v.len() - 1], OxideLayer::KeepOut);
    }
}
