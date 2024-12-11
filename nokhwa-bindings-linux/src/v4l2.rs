use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use v4l::{Device, Format, FourCC, Fraction};
use v4l2_sys_mit::{V4L2_CID_AUTO_WHITE_BALANCE, V4L2_CID_BACKLIGHT_COMPENSATION, V4L2_CID_BRIGHTNESS, V4L2_CID_CONTRAST, V4L2_CID_DO_WHITE_BALANCE, V4L2_CID_EXPOSURE, V4L2_CID_FOCUS_ABSOLUTE, V4L2_CID_FOCUS_RELATIVE, V4L2_CID_GAIN, V4L2_CID_GAMMA, V4L2_CID_HUE, V4L2_CID_HUE_AUTO, V4L2_CID_IRIS_ABSOLUTE, V4L2_CID_IRIS_RELATIVE, V4L2_CID_PAN_ABSOLUTE, V4L2_CID_PAN_RELATIVE, V4L2_CID_SATURATION, V4L2_CID_SHARPNESS, V4L2_CID_TILT_ABSOLUTE, V4L2_CID_TILT_RELATIVE, V4L2_CID_WHITE_BALANCE_TEMPERATURE, V4L2_CID_ZOOM_ABSOLUTE, V4L2_CID_ZOOM_CONTINUOUS, V4L2_CID_ZOOM_RELATIVE};
use v4l::device::Handle;
use v4l::frameinterval::FrameIntervalEnum;
use v4l::prelude::MmapStream;
use v4l::video::{Capture as V4lCapture, Output};
use v4l::video::output::Parameters;
use nokhwa_core::frame_buffer::FrameBuffer;
use nokhwa_core::camera::{Camera, Open, Setting, Capture};
use nokhwa_core::properties::{CameraProperties, CameraPropertyFlag, CameraPropertyId, CameraPropertyValue};
use nokhwa_core::{define_back_and_fourth_control, define_back_and_fourth_frame_format};
use nokhwa_core::error::{NokhwaError, NokhwaResult};
use nokhwa_core::frame_format::FrameFormat;
use nokhwa_core::types::{CameraFormat, CameraIndex, CameraInformation, FrameRate, Resolution};

const NULL_FCC: &'static [u8; 4] = &[0x00, 0x00, 0x00, 0x00];

pub use v4l2_sys_mit::*;
pub use v4l::*;

fn func_u8_8_to_fcc(u8_8: [u8; 8]) -> FrameFormatIntermediate {
    FrameFormatIntermediate([u8_8[0], u8_8[1], u8_8[2], u8_8[3]])
}

fn func_fcc_to_u8_8(u8_4: [u8; 4]) -> [u8; 8] {
    [u8_4[0], u8_4[1], u8_4[2], u8_4[3], 0x00, 0x00, 0x00, 0x00]
}

fn value_to_fcc_type(v: &[u8; 4]) -> [u8;4] {
    *v
}

define_back_and_fourth_frame_format!([u8;4], {
    FrameFormat::H265 => b"HEVC",
    FrameFormat::H264 => b"H264",
    FrameFormat::H263 => b"H263",
    FrameFormat::Av1 => b"AV1F",
    FrameFormat::Avc1 => b"AVC1",
    FrameFormat::Mpeg1 => b"MPG1",
    FrameFormat::Mpeg2 => b"MPG2",
    FrameFormat::Mpeg4 => b"MPG4",
    FrameFormat::MJpeg => b"MJPG",
    FrameFormat::XVid => b"XVID",
    FrameFormat::VP8 => b"VP80",
    FrameFormat::VP9 => b"VP90",
    FrameFormat::Ayuv444 => b"AYUV",
    FrameFormat::Yuyv422 => b"YUYV",
    FrameFormat::Uyvy422 => b"UYVY",
    FrameFormat::Yvyu422 => b"YVYU",
    FrameFormat::Nv12 => b"NV12",
    FrameFormat::Nv21 => b"NV21",
    FrameFormat::Yv12 => b"YV12",
    FrameFormat::I420 => b"YU12",
    FrameFormat::Yvu9 => b"YVU9",
    FrameFormat::Luma8 => b"GREY",
    FrameFormat::Luma16 => b"Y16 ",
    FrameFormat::Depth16 => b"Z16 ",
    FrameFormat::Rgb332 => b"RGB3",
    FrameFormat::RgbA8888 => b"AB24",
    FrameFormat::ARgb8888 => b"BA24",
    FrameFormat::Rgb555 => b"RX15",
    FrameFormat::Rgb565 => b"RGBP",
    FrameFormat::Bayer8 => b"BA81",
    FrameFormat::Bayer16 => b"BYR2",
}, func_u8_8_to_fcc, func_fcc_to_u8_8, value_to_fcc_type);

fn linux_id_to_str(id: u32) -> String {
    id.to_string()
}

fn str_to_linux_id(id: &str) -> Option<u32> {
    u32::from_str(id).ok()
}

define_back_and_fourth_control!(u32, {
    CameraPropertyId::BacklightCompensation, None => V4L2_CID_BACKLIGHT_COMPENSATION,
    CameraPropertyId::Brightness, None => V4L2_CID_BRIGHTNESS,
    CameraPropertyId::Contrast, None => V4L2_CID_CONTRAST,
    CameraPropertyId::Exposure, None => V4L2_CID_EXPOSURE,
    CameraPropertyId::Focus, Some(CameraPropertyFlag::Relative) => V4L2_CID_FOCUS_RELATIVE,
    CameraPropertyId::Focus, Some(CameraPropertyFlag::Absolute) => V4L2_CID_FOCUS_ABSOLUTE,
    CameraPropertyId::Gamma, None => V4L2_CID_GAMMA,
    CameraPropertyId::Gain, None => V4L2_CID_GAIN,
    CameraPropertyId::Hue, None => V4L2_CID_HUE,
    CameraPropertyId::Hue, Some(CameraPropertyFlag::Automatic) => V4L2_CID_HUE_AUTO,
    CameraPropertyId::Iris, Some(CameraPropertyFlag::Relative) => V4L2_CID_IRIS_RELATIVE,
    CameraPropertyId::Iris, Some(CameraPropertyFlag::Absolute) => V4L2_CID_IRIS_ABSOLUTE,
    CameraPropertyId::Saturation, None => V4L2_CID_SATURATION,
    CameraPropertyId::Sharpness, None => V4L2_CID_SHARPNESS,
    CameraPropertyId::Pan, Some(CameraPropertyFlag::Absolute) => V4L2_CID_PAN_ABSOLUTE,
    CameraPropertyId::Pan, Some(CameraPropertyFlag::Relative) => V4L2_CID_PAN_RELATIVE,
    // CameraPropertyId::Pan, None => V4L2_CID_PAN_ABSOLUTE,
    // CameraPropertyId::Tilt, None => V4L2_CID_TILT_ABSOLUTE,
    CameraPropertyId::Tilt, Some(CameraPropertyFlag::Absolute) => V4L2_CID_TILT_ABSOLUTE,
    CameraPropertyId::Tilt, Some(CameraPropertyFlag::Relative) => V4L2_CID_TILT_RELATIVE,
    // CameraPropertyId::Zoom, None => V4L2_CID_ZOOM_ABSOLUTE,
    CameraPropertyId::WhiteBalance, None => V4L2_CID_WHITE_BALANCE_TEMPERATURE,
    CameraPropertyId::WhiteBalance, Some(CameraPropertyFlag::Automatic) => V4L2_CID_AUTO_WHITE_BALANCE,
    CameraPropertyId::WhiteBalance, Some(CameraPropertyFlag::Enable) => V4L2_CID_DO_WHITE_BALANCE,
    CameraPropertyId::Zoom, Some(CameraPropertyFlag::Absolute) => V4L2_CID_ZOOM_ABSOLUTE,
    CameraPropertyId::Zoom, Some(CameraPropertyFlag::Relative) => V4L2_CID_ZOOM_RELATIVE,
    CameraPropertyId::Zoom, Some(CameraPropertyFlag::Continuous) => V4L2_CID_ZOOM_CONTINUOUS,
    // CameraPropertyId::Iris, None => V4L2_CID_IRIS_ABSOLUTE,

}, linux_id_to_str, str_to_linux_id);

pub struct DeviceInner {
    device: Device,
}

impl DeviceInner {
    pub fn new(index: usize) -> Result<Self, NokhwaError> {
        let device = Device::new(index).map_err(|why| NokhwaError::OpenDeviceError(index.to_string(), why.to_string()))?;
        Ok(DeviceInner { device })
    }


    pub fn resolutions(&self, fourcc: FourCC) -> Result<Vec<Resolution>, NokhwaError> {
        let resolutions = self.device.enum_framesizes(fourcc.into()).map_err(|why| NokhwaError::GetPropertyError { property: "enum_framesizes".to_string(), error: why.to_string() })?.into_iter().map(|r| r.size.to_discrete().into_iter()).flatten().map(|res| Resolution::new(res.width, res.height) ).collect::<Vec<Resolution>>();
        Ok(resolutions)
    }

    pub fn frame_rates(&self, fourcc: FourCC, resolution: Resolution) -> Result<Vec<FrameRate>, NokhwaError> {
        let frame_rates = match self.device.enum_frameintervals(fourcc, resolution.width(), resolution.height()) {
            Ok(intervals) => intervals.into_iter().map(|x| match x.interval {
                FrameIntervalEnum::Discrete(d) => vec![d],
                FrameIntervalEnum::Stepwise(step) => {
                    let mut temp_vec = vec![];
                    for rate in (step.min.numerator..step.max.numerator).step_by(step.step.numerator as usize) {
                        temp_vec.push(Fraction::new(rate, step.min.denominator))
                    }
                    temp_vec
                }
            }),
            Err(why) => {
                return Err(NokhwaError::GetPropertyError { property: "enum_frameintervals".to_string(), error: why.to_string() })
            }
        }.into_iter().flatten().map(|x| FrameRate::new(x.numerator, x.denominator)).collect::<Vec<FrameRate>>();
        Ok(frame_rates)
    }

    pub fn properties(&self) -> CameraProperties {

    }

    pub fn inner(&self) -> &Device {
        &self.device
    }
}

pub struct StreamInner<'a> {
    stream: MmapStream<'a>
}
