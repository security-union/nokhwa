use std::collections::HashMap;
use crate::ranges::Range;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlId {
    Focus,
    Exposure,
    WhiteBalance,
    Zoom,
    Lighting,
    Other(u64)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlGroup {
    ModeMultipleValue(ModeAndValuesControl),
    Simple(SimpleControl),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModeAndValuesControl {
    id: ControlId,
    mode_id: u64,
    mode_body: ControlBody,
    values: HashMap<String, SimpleControl>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleControl {
    id: u64,
    body: ControlBody,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlBody {
    pub typ: ControlType,
    pub class: ControlClass,
    pub flags: Vec<ControlFlags>,
    pub descriptor: ControlValueDescriptor,
    pub value: Option<ControlValue>
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlType {
    Button,
    Integer,
    Menu,
    IntegerMenu,
    BinaryMenu,
    Bitmask,
    String,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlClass {
    User,
    Camera,
    Other(u64),
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlFlags {
    Disabled,
    Busy,
    ReadOnly,
    CascadingUpdates,
    Inactive,
    Slider,
    WriteOnly,
    ContinuousChange,
    ExecuteOnWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlValueDescriptor {
    Null,
    Integer(Range<i64>),
    Bitmap(i64),
    Float(Range<i64>),
    String(String),
    Boolean(bool),
    Array(Vec<ControlValuePrimitive>),
    Map(HashMap<String, ControlValuePrimitive>)
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ControlValuePrimitive {
    Null,
    Integer(i64),
    Bitmap(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ControlValue {
    Null,
    Integer(i64),
    Bitmap(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    KeyValue(String, ControlValuePrimitive),
}
