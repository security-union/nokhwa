use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::{ControlFlow};
use crate::error::{NokhwaError, NokhwaResult};
use crate::ranges::{Range, ValidatableRange};

pub type PlatformSpecificControlId = u64;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlId {
    FocusMode,
    FocusAutoType,
    FocusAutoRange,
    FocusAbsolute,
    FocusRelative,
    FocusStatus,

    ExposureMode,
    ExposureBias,
    ExposureTime,
    ExposureAutoPriority,
    ExposureIsoMode,
    ExposureIsoSensitivity,
    ExposureApertureAbsolute,
    ExposureApertureRelative,

    WhiteBalanceMode,
    WhiteBalanceTemperature,

    ZoomMode,
    LightingMode,
    PlatformSpecific(PlatformSpecificControlId)
}

impl Display for ControlId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Control ID: {self:?}")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Properties {
    controls: HashMap<ControlId, ControlBody>,
}

impl Properties {
    pub fn new(device_controls: HashMap<ControlId, ControlBody>) -> Self {
        Self {
            controls: device_controls,
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn control_value(&self, control_id: &ControlId) -> Option<&ControlBody> {
        self.controls.get(control_id)
    }

    pub fn set_control_value(&mut self, control_id: &ControlId, value: ControlValue) -> NokhwaResult<()> {
        // see if it exists
        if let Some(control) = self.controls.get_mut(control_id) {
            // FIXME: Remove this clone one day!
            control.set_value(value.clone())?;
        }
        Err(NokhwaError::SetPropertyError {
            property: control_id.to_string(),
            value: value.to_string(),
            error: "Not Found/Not Supported".to_string(),
        })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ControlBody {
    control_type: ControlType,
    flags: HashSet<ControlFlags>,
    descriptor: ControlValueDescriptor,
    value: Option<ControlValue>,
    default_value: Option<ControlValue>,
}

impl ControlBody {
    pub fn new(control_type: ControlType, control_flags: HashSet<ControlFlags>, control_value_descriptor: ControlValueDescriptor, value: Option<ControlValue>, default_value: Option<ControlValue>) -> Self {
        Self {
            control_type,
            flags: control_flags,
            descriptor: control_value_descriptor,
            value,
            default_value,
        }
    }

    pub fn control_type(&self) -> &ControlType {
        &self.control_type
    }

    pub fn flags(&self) -> &HashSet<ControlFlags> {
        &self.flags
    }

    pub fn descriptor(&self) -> &ControlValueDescriptor {
        &self.descriptor
    }

    pub fn value(&self) -> &Option<ControlValue> {
        &self.value
    }

    pub fn default_value(&self) -> &Option<ControlValue> {
        &self.default_value
    }

    pub fn add_flag(&mut self, flag: ControlFlags) {
        self.flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: ControlFlags) -> bool {
        self.flags.remove(&flag)
    }

    pub fn set_value(&mut self, value: ControlValue) -> NokhwaResult<Option<ControlValue>> {
        if let ControlFlow::Break(()) =  self.descriptor.validate(&value) {
            return Err(NokhwaError::SetPropertyError {
                property: "Control Body".to_string(),
                value: value.to_string(),
                error: "Failed to validate control value".to_string(),
            })
        }

        let old = core::mem::replace(&mut self.value, Some(value));
        Ok(old)
    }

    pub fn clear_value(&mut self) -> Option<ControlValue> {
        core::mem::replace(&mut self.value, None)
    }


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
    BitMask,
    Float(Range<f64>),
    String,
    Boolean,
    // Array of any values of singular type
    Array(ControlValuePrimitiveDescriptor),
    // Multiple Choice from array
    MultiChoice(Vec<ControlValuePrimitiveDescriptor>),
    // Singular Choice from array
    Enum(Vec<ControlValuePrimitiveDescriptor>),
    // Hashmap
    Map(HashMap<String, ControlValuePrimitiveDescriptor>),
    // A menu, where you pick a key-value
    Menu(HashMap<String, ControlValuePrimitiveDescriptor>)
}

impl ControlValueDescriptor {
    pub fn validate(&self, value: &ControlValue) -> ControlFlow<()> {
        match self {
            ControlValueDescriptor::Null => {
                if let &ControlValue::Null = value {
                    return ControlFlow::Continue(())
                }
            }
            ControlValueDescriptor::Integer(int_range) => {
                if let ControlValue::Integer(i) = value {
                    int_range.validate(i)?;
                }
            }
            ControlValueDescriptor::BitMask => {
                if let &ControlValue::BitMask(_) = value {
                    return ControlFlow::Continue(())
                }
            }
            ControlValueDescriptor::Float(float_range) => {
                if let ControlValue::Float(i) = value {
                    float_range.validate(i)?;
                }
            }
            ControlValueDescriptor::String => {
                if let &ControlValue::String(_) = value {
                    return ControlFlow::Continue(())
                }
            }
            ControlValueDescriptor::Boolean => {
                if let &ControlValue::Boolean(_) = value {
                    return ControlFlow::Continue(())
                }
            }
            ControlValueDescriptor::Array(arr) => {
                if arr.is_valid_value(value) {
                    return ControlFlow::Continue(())
                }
            }
            ControlValueDescriptor::MultiChoice(choices) => {
                if let &ControlValue::Array(values) = value {
                    for v in values {
                        let mut contains = false;
                        for choice in choices {
                            if choice.is_valid_value(v.as_ref()) {
                                contains = true;
                                break;
                            }
                        }
                        if !contains {
                            return ControlFlow::Break(())
                        }
                    }
                }
            }
            ControlValueDescriptor::Enum(choices) => {
                for choice in choices {
                    if choice.is_valid_value(&value) {
                        return ControlFlow::Continue(())
                    }
                }
            }
            ControlValueDescriptor::Map(map) => {
                if let ControlValue::Map(setting_map) = &value {
                    for (setting_key, setting_value) in setting_map {
                        if let Some(descriptor) = map.get(setting_key) {
                            if !descriptor.is_valid_value(setting_value.as_ref()) {
                                return ControlFlow::Break(())
                            }
                        }
                    }
                }
            }
            ControlValueDescriptor::Menu(menu) => {
                if let ControlValue::KeyValue(k, v) = &value {
                    if let Some(descriptor) = menu.get(k) {
                        if descriptor.is_valid_value(v.as_ref()) {
                            return ControlFlow::Continue(())
                        }
                    }
                }
            }
        }

        ControlFlow::Break(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlValuePrimitiveDescriptor {
    Null,
    Integer(Range<i64>),
    BitMask,
    Float(Range<f64>),
    String,
    Boolean,
}

impl ControlValuePrimitiveDescriptor {
    pub fn is_valid_value(&self, other: &ControlValue) -> bool {
        match self {
            ControlValuePrimitiveDescriptor::Null => {
                if let &ControlValue::Null = other {
                    return true
                }
            }
            ControlValuePrimitiveDescriptor::Integer(int_range) => {
                if let ControlValue::Integer(i) = other {
                    return int_range.validate(i).is_ok()
                }
            }
            ControlValuePrimitiveDescriptor::BitMask => {
                if let &ControlValue::BitMask(_) = other {
                    return true
                }
            }
            ControlValuePrimitiveDescriptor::Float(float_range) => {
                if let ControlValue::Float(i) = other {
                    return float_range.validate(i).is_ok()
                }
            }
            ControlValuePrimitiveDescriptor::String => {
                if let &ControlValue::String(_) = other {
                    return true
                }
            }
            ControlValuePrimitiveDescriptor::Boolean => {
                if let &ControlValue::Boolean(_) = other {
                    return true
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ControlValuePrimitive {
    Null,
    Integer(i64),
    BitMask(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl AsRef<ControlValue> for ControlValuePrimitive {
    fn as_ref(&self) -> &ControlValue {
        match self {
            ControlValuePrimitive::Null => &ControlValue::Null,
            ControlValuePrimitive::Integer(i) => &ControlValue::Integer(*i),
            ControlValuePrimitive::BitMask(b) => &ControlValue::BitMask(*b),
            ControlValuePrimitive::Float(f) => &ControlValue::Float(*f),
            ControlValuePrimitive::String(s) => &ControlValue::String(s.clone()),
            ControlValuePrimitive::Boolean(b) => &ControlValue::Boolean(*b),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlValue {
    Null,
    Integer(i64),
    BitMask(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<ControlValuePrimitive>),
    KeyValue(String, ControlValuePrimitive),
    Map(HashMap<String, ControlValuePrimitive>),
}

impl ControlValue {
    pub fn same_type(&self, other: &ControlValue) -> bool {
        match self {
            ControlValue::Null => {
                if let ControlValue::Null = other {
                    return true;
                }
            }
            ControlValue::Integer(_) => {if let ControlValue::Integer(_) = other {
                return true;
            }}
            ControlValue::BitMask(_) => {if let ControlValue::BitMask(_) = other {
                return true;
            }}
            ControlValue::Float(_) => {if let ControlValue::Float(_) = other {
                return true;
            }}
            ControlValue::String(_) => {if let ControlValue::String(_) = other {
                return true;
            }}
            ControlValue::Boolean(_) => {if let ControlValue::Boolean(_) = other {
                return true;
            }}
            ControlValue::Array(_) => {if let ControlValue::Array(_) = other {
                return true;
            }}
            ControlValue::KeyValue(_, _) => {if let ControlValue::KeyValue(_, _) = other {
                return true;
            }}
            ControlValue::Map(_) => {if let ControlValue::Map(_) = other {
                return true;
            }}
        }

        false
    }
}

impl Display for ControlValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Control Value: {self:?}")
    }
}

impl From<ControlValuePrimitive> for ControlValue {
    fn from(value: ControlValuePrimitive) -> Self {
        match value {
            ControlValuePrimitive::Null => ControlValue::Null,
            ControlValuePrimitive::Integer(i) => ControlValue::Integer(i),
            ControlValuePrimitive::BitMask(b) => ControlValue::BitMask(b),
            ControlValuePrimitive::Float(f) => ControlValue::Float(f),
            ControlValuePrimitive::String(s) => ControlValue::String(s),
            ControlValuePrimitive::Boolean(b) => ControlValue::Boolean(b),
        }
    }
}
