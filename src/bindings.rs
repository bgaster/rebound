//! Description: 
//! 
//! Input bindings
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use std::fmt::{self, Display};

use amethyst::{
    input::{BindingTypes},
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    ToggleMenu,
    OpenTheme,
    ResetTheme,
    FileNew,
    FileOpen,
    FileSave,
    FileExportVector,
    FileExportImage,
    EditUndo,
    EditRedo,
    ViewColourPicker,
    ViewToggleGrid,
    ViewToggleTools,
    LayersForeground,
    LayersMiddleground,
    LayersBackground,
    LayersMergeLayers,
    StrokeLine,
    StrokeArc,
    StrokeArcRev,
    StrokeBezier,
    StrokeClose,
    StrokeArcFull,
    StrokeArcRevFull,
    StrokeClearSelection,
    StrokeEraseSegment,
    ControlAddPoint,
    ControlMoveUp,
    ControlMoveRight,
    ControlMoveDown,
    ControlMoveLeft,
    ControlRemovePoint,
    StyleLineCap,
    StyleLineJoin,
    StyleMirror,
    StyleFill,
    StyleThicker,
    StyleThinner,
    StyleThicker5,
    StyleThinner5,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InputBindingTypes;

impl BindingTypes for InputBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}