use crate::error::{NokhwaError, NokhwaResult};
use crate::frame_buffer::FrameBuffer;
use crate::frame_format::FrameFormat;
use crate::properties::{CameraProperties, CameraPropertyId, CameraPropertyValue};
use crate::types::{CameraFormat, CameraIndex, FrameRate, Resolution};
use std::collections::HashMap;
use crate::stream::Stream;

pub trait Open {
    fn open(index: CameraIndex) -> NokhwaResult<Self> where Self: Sized;
}

#[cfg(feature = "async")]
pub trait AsyncOpen: Sized {
    async fn open_async(index: CameraIndex) -> NokhwaResult<Self>;
}

macro_rules! def_camera_props {
    ( $($property:ident, )* ) => {
        paste::paste! {
            $(
            fn [<$property:snake>] (&self) -> Option<&crate::properties::CameraPropertyDescriptor> {
                self.properties().[<$property:snake>]()
            }

            fn [<set_ $property:snake>]  (&mut self, value: crate::properties::CameraPropertyValue) -> Result<(), crate::error::NokhwaError> {
                self.set_property(&crate::properties::CameraPropertyId::$property, value)
            }
            )*
        }
    };
}

// macro_rules! def_camera_props_async {
//     ( $($property:ident, )* ) => {
//         paste::paste! {
//             $(
//                 async fn [<set_ $property:snake _async>] (&mut self, value: CameraPropertyValue) -> Result<(), NokhwaError> {
//                 self.[<set_ $property:snake >](value)
//             }
//             )*
//         }
//     };
// }

pub trait Setting {
    fn enumerate_formats(&self) -> Result<Vec<CameraFormat>, NokhwaError>;

    fn enumerate_resolution_and_frame_rates(
        &self,
        frame_format: FrameFormat,
    ) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError>;

    fn set_format(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;

    fn properties(&self) -> &CameraProperties;

    fn set_property(
        &mut self,
        property: &CameraPropertyId,
        value: CameraPropertyValue,
    ) -> Result<(), NokhwaError>;

    def_camera_props!(
        Brightness,
        Contrast,
        Hue,
        Saturation,
        Sharpness,
        Gamma,
        WhiteBalance,
        BacklightCompensation,
        Pan,
        Tilt,
        Zoom,
        Exposure,
        Iris,
        Focus,
        Facing,
    );
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
