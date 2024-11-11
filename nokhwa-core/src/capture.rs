use std::collections::HashMap;
use crate::buffer::Buffer;
use crate::properties::{CameraProperties, CameraPropertyId, CameraPropertyValue};
use crate::error::{NokhwaError, NokhwaResult};
use crate::frame_format::FrameFormat;
use crate::stream::CaptureStream;
use crate::types::{CameraFormat, CameraIndex, FrameRate, Resolution};

pub trait Open {
    fn open(index: CameraIndex) -> NokhwaResult<Self>; 
}

#[cfg(feature = "async")]
pub trait AsyncOpen {
    async fn open_async(index: CameraIndex) -> NokhwaResult<Self>;
}

macro_rules! def_camera_props {
    ( $($property:ident, )* ) => {
        paste::paste! {
            $(
            fn [<$property:snake>] (&self) -> Option<&CameraPropertyDescriptor> {
                self.properties().[<$property:snake>]
            }
            
            fn [<set_ $property:snake>]  (&mut self, value: CameraPropertyValue) -> Result<(), NokhwaError> {
                self.properties().[<set_ $property:snake >](value)
            }
            )*
        }
    };
}

macro_rules! def_camera_props_async {
    ( $($property:ident, )* ) => {
        paste::paste! {
            $(
                async fn [<set_ $property:snake _async>] (&mut self, value: CameraPropertyValue) -> Result<(), NokhwaError> {
                self.properties().[<set_ $property:snake >](value)
            }
            )*
        }
    };
}

pub trait Setting {
    fn enumerate_formats(&self) -> Result<Vec<CameraFormat>, NokhwaError>;
    
    fn enumerate_resolution_and_frame_rates(&self, frame_format: FrameFormat) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError>;
    
    fn set_format(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;
    
    fn properties(&self) -> &CameraProperties;
    
    fn set_property(&mut self, property: &CameraPropertyId, value: CameraPropertyValue) -> Result<(), NokhwaError>; 

    def_camera_props!(
        Brightness,
        Contrast,
        Hue,
        Saturation,
        Sharpness,
        Gamma,
        WhiteBalance,
        BacklightCompensation,
        Gain,
        Pan,
        Tilt,
        Zoom,
        Exposure,
        Iris,
        Focus,
        Facing, 
    );
}

#[cfg(feature = "async")]
pub trait AsyncSetting {
    async fn set_format_async(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;

    async fn set_property_async(&mut self, property: &CameraPropertyId, value: CameraPropertyValue) -> Result<(), NokhwaError>;

    def_camera_props_async!(
        Brightness,
        Contrast,
        Hue,
        Saturation,
        Sharpness,
        Gamma,
        WhiteBalance,
        BacklightCompensation,
        Gain,
        Pan,
        Tilt,
        Zoom,
        Exposure,
        Iris,
        Focus,
        Facing, 
    );
}

pub trait Stream {
    fn open_stream(&mut self) -> Result<CaptureStream, NokhwaError>;

    fn close_stream(&mut self) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
pub trait AsyncStream {
    async fn open_stream(&mut self) -> Result<(), NokhwaError>;

    async fn await_frame(&mut self) -> Result<Buffer, NokhwaError>;

    async fn close_stream(&mut self) -> Result<(), NokhwaError>;}

pub trait Capture: Open + Setting + Stream {}

#[cfg(feature = "async")]
pub trait AsyncCapture: Capture + AsyncOpen + AsyncSetting + AsyncStream {}


