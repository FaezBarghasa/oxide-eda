//! Strongly-typed application menu commands, editor contexts, and keybindings.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Hierarchical editor context for scoping commands and keybindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EditorContext {
    #[default]
    Global,
    Schematic,
    Pcb,
    Library,
    OutputJob,
    CamEditor,
    TextEditor,
}

/// Unified menu command enum covering all document and system actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "category", content = "command", rename_all = "snake_case")]
pub enum MenuCommand {
    File(FileMenuCommand),
    Edit(EditMenuCommand),
    View(ViewMenuCommand),
    Project(ProjectMenuCommand),
    Place(PlaceCommand),
    Design(DesignCommand),
    Tools(ToolsMenuCommand),
    Reports(ReportsMenuCommand),
    Window(WindowMenuCommand),
    Help(HelpMenuCommand),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileMenuCommand {
    NewProject,
    NewSchematic,
    NewPcb,
    NewLibrary,
    NewOutputJob,
    Open(Option<PathBuf>),
    OpenRecent(PathBuf),
    Save,
    SaveAs(Option<PathBuf>),
    SaveAll,
    Print,
    PrintPreview,
    Close,
    CloseAll,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditMenuCommand {
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Delete,
    SelectAll,
    DeselectAll,
    SelectNet,
    FindSimilarObjects,
    RotateSelection(f64),
    MirrorSelectionHorizontal,
    MirrorSelectionVertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewMenuCommand {
    ZoomIn,
    ZoomOut,
    ZoomToFit,
    ZoomToSelection,
    ToggleGrid,
    ToggleRulers,
    ToggleAllPanels,
    ToggleSingleLayerMode,
    CycleLayer,
    Toggle3dView,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectMenuCommand {
    ValidateProject,
    CompileProject,
    ProjectOptions,
    AddExistingDocument(PathBuf),
    AddNewDocument,
    ShowDifferences,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "target", content = "action", rename_all = "snake_case")]
pub enum PlaceCommand {
    Schematic(SchematicPlaceCommand),
    Pcb(PcbPlaceCommand),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchematicPlaceCommand {
    Wire,
    Bus,
    BusEntry,
    NetLabel,
    PowerPort(String),
    NoConnect,
    Component,
    Port,
    Text,
    Line,
    Rectangle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PcbPlaceCommand {
    Track,
    InteractiveRoute,
    Via,
    Pad,
    PolygonPour,
    Fill,
    Text,
    Dimension,
    Line,
    Arc,
    Room,
    Keepout,
    DifferentialPair,
    LengthTuning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "target", content = "action", rename_all = "snake_case")]
pub enum DesignCommand {
    Schematic(SchematicDesignCommand),
    Pcb(PcbDesignCommand),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchematicDesignCommand {
    CreateNetlist,
    UpdatePcb,
    AnnotateSchematic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PcbDesignCommand {
    ImportChangesFromSchematic,
    ExportChangesToSchematic,
    LayerStackupManager,
    Rules,
    NetClasses,
    Rooms,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolsMenuCommand {
    Annotation,
    FootprintManager,
    DesignRuleCheck,
    ElectricalRuleCheck,
    UpdateFromLibrary,
    SetGrid,
    Preferences,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportsMenuCommand {
    BillOfMaterials,
    BoardInformation,
    DrillReport,
    NetlistReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowMenuCommand {
    NextDocument,
    PreviousDocument,
    CloseDocument,
    TileHorizontal,
    TileVertical,
    ResetLayout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpMenuCommand {
    Documentation,
    ContextSensitiveHelp,
    KeyboardShortcuts,
    About,
}

/// Keyboard key representation for bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Asterisk,
    Plus,
    Minus,
    Slash,
    Backslash,
}

/// Modifier keys (Ctrl/Cmd, Shift, Alt).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
    };
    pub const CTRL: Self = Self {
        ctrl: true,
        shift: false,
        alt: false,
        meta: false,
    };
    pub const SHIFT: Self = Self {
        ctrl: false,
        shift: true,
        alt: false,
        meta: false,
    };
    pub const ALT: Self = Self {
        ctrl: false,
        shift: false,
        alt: true,
        meta: false,
    };
    pub const CTRL_SHIFT: Self = Self {
        ctrl: true,
        shift: true,
        alt: false,
        meta: false,
    };
}

/// Complete key binding representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: KeyCode,
    #[serde(default)]
    pub modifiers: Modifiers,
    #[serde(default)]
    pub context: EditorContext,
}

impl KeyBinding {
    pub fn new(key: KeyCode, modifiers: Modifiers, context: EditorContext) -> Self {
        Self {
            key,
            modifiers,
            context,
        }
    }

    pub fn global(key: KeyCode, modifiers: Modifiers) -> Self {
        Self::new(key, modifiers, EditorContext::Global)
    }
}
