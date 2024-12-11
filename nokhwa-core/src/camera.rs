use crate::error::{NokhwaError, NokhwaResult};
use crate::frame_format::FrameFormat;
use crate::properties::{ControlId, ControlValue, Properties};
use crate::types::{CameraFormat, CameraIndex, FrameRate, Resolution};
use std::collections::HashMap;
use crate::frame_buffer::FrameBuffer;
use crate::stream::Stream;

pub trait Open {
    fn open(index: CameraIndex) -> NokhwaResult<Self> where Self: Sized;
}

#[cfg(feature = "async")]
pub trait AsyncOpen: Sized {
    async fn open_async(index: CameraIndex) -> NokhwaResult<Self>;
}


pub trait Setting {
    fn enumerate_formats(&self) -> Result<Vec<CameraFormat>, NokhwaError>;

    fn enumerate_resolution_and_frame_rates(
        &self,
        frame_format: FrameFormat,
    ) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError>;

    fn set_format(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;

    fn properties(&self) -> &Properties;

    fn set_property(
        &mut self,
        property: &ControlId,
        value: ControlValue,
    ) -> Result<(), NokhwaError>;
}

// #[cfg(feature = "async")]
// pub trait AsyncSetting {
//     async fn set_format_async(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;
// 
//     async fn set_property_async(
//         &mut self,
//         property: &CameraPropertyId,
//         value: CameraPropertyValue,
//     ) -> Result<(), NokhwaError>;
// 
//     def_camera_props_async!(
//         Brightness,
//         Contrast,
//         Hue,
//         Saturation,
//         Sharpness,
//         Gamma,
//         WhiteBalance,
//         BacklightCompensation,
//         Pan,
//         Tilt,
//         Zoom,
//         Exposure,
//         Iris,
//         Focus,
//         Facing,
//     );
// }

pub trait Capture {
    fn open_stream(&mut self) -> Result<Stream, NokhwaError>;

    fn close_stream(&mut self) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
pub trait AsyncStream {
    async fn open_stream(&mut self) -> Result<(), NokhwaError>;

    async fn await_frame(&mut self) -> Result<FrameBuffer, NokhwaError>;

    async fn close_stream(&mut self) -> Result<(), NokhwaError>;
}

pub trait CameraVtable: Setting + Capture {}

pub trait Camera: Open + CameraVtable {}

#[cfg(feature = "async")]
pub trait AsyncCapture: Camera + AsyncOpen + AsyncStream {}
