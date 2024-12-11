use std::collections::HashMap;
use std::sync::Arc;
use nokhwa_bindings_linux::{
    v4l2::{
        DeviceInner,
        FrameFormatIntermediate,
        format::{Format, FourCC},
        fraction::Fraction,
        video::{
            Capture,
            capture::Parameters
        }
    }
};
use nokhwa_core::{
    camera::{Open, Setting},
    error::{NokhwaError, NokhwaResult},
    frame_format::FrameFormat,
    properties::CameraProperties,
    types::{CameraFormat, CameraIndex, CameraInformation, FrameRate, Resolution}
};

pub struct V4L2CaptureDevice {
    device_inner: Arc<DeviceInner>,
    camera_info: CameraInformation,
    format: Option<CameraFormat>,
    properties: Option<CameraProperties>,
    stream_running: bool,
}

impl Open for V4L2CaptureDevice {
    fn open(index: CameraIndex) -> NokhwaResult<Self> {
        let device = DeviceInner::new(index.as_index()? as usize).map_err(|why| NokhwaError::OpenDeviceError(index.to_string(), why.to_string()))?;
        let caps = device.inner().query_caps().map_err(|why| NokhwaError::OpenDeviceError(index.to_string(), why.to_string()))?;
        let camera_info = CameraInformation::new(caps.card, caps.bus, caps.driver, index);
        Ok(Self {
            device_inner: Arc::new(device),
            camera_info,
            format: None,
            properties: None,
            stream_running: false,
        })
    }
}

impl Setting for V4L2CaptureDevice {
    fn enumerate_formats(&self) -> Result<Vec<CameraFormat>, NokhwaError> {
        let formats_fourcc = self.device_inner.inner().enum_formats().map_err(|why| NokhwaError::GetPropertyError { property: "enum_formats".to_string(), error: why.to_string() })?.into_iter().map(|desc| desc.fourcc).collect::<Vec<FourCC>>();
        let mut camera_formats = vec![];

        for fourcc in formats_fourcc {
            let frame_format = FrameFormatIntermediate::into_frame_format(fourcc.repr);
            for resolution in self.resolutions(fourcc)? {
                for frame_rate in self.frame_rates(fourcc, resolution)? {
                    camera_formats.push(
                        CameraFormat::new(resolution, frame_format, frame_rate)
                    );
                }
            }
        }
        Ok(camera_formats)
    }

    fn enumerate_resolution_and_frame_rates(&self, frame_format: FrameFormat) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError> {
        let fourcc = match FrameFormatIntermediate::from_frame_format(frame_format) {
            Some(v) => v,
            None => return Err(NokhwaError::GetPropertyError { property: "enumerate_resolution_and_frame_rates".to_string(), error: "Unsupported FourCC".to_string() }),
        };
        let mut resolutions_and_frame_rates = HashMap::new();
        for resolution in self.device_inner.resolutions(fourcc.0.into())? {
            let frame_rates = self.device_inner.frame_rates(fourcc.0.into(), resolution)?;
            resolutions_and_frame_rates.insert(resolution, frame_rates);
        }

        Ok(resolutions_and_frame_rates)
    }

    fn set_format(&self, camera_format: CameraFormat) -> Result<(), NokhwaError> {
        let fourcc = match FrameFormatIntermediate::from_frame_format(camera_format.format()) {
            Some(v) => v,
            None => return Err(NokhwaError::GetPropertyError { property: "set_format".to_string(), error: "Unsupported FourCC".to_string() }),
        };

        let format = Format::new(camera_format.width(), camera_format.height(), FourCC::new(&fourcc.0));

        let frame_rate = Fraction::new(camera_format.frame_rate().numerator(), camera_format.frame_rate().denominator());

        self.device_inner.inner().set_format(&format).map_err(|why| {
            Err(NokhwaError::SetPropertyError {
                property: "set_format".to_string(),
                value: camera_format.to_string(),
                error: why.to_string(),
            })
        })?;

        self.device_inner.inner().set_params(&Parameters::new(frame_rate)).map_err(|why| {
            Err(NokhwaError::SetPropertyError {
                property: "set_params".to_string(),
                value: camera_format.to_string(),
                error: why.to_string(),
            })
        })?;
    }

    fn properties(&self) -> &CameraProperties {
        let ctrls = self.device_inner.inner().query_controls().map_err(|why| {
            Err(NokhwaError::GetPropertyError { property: "query_controls".to_string(), error: why.to_string() })
        })?.into_iter().map(|desc| {
            match v4l2_sys_mit::
        });

    }

    fn set_property(&mut self, property: &CameraPropertyId, value: CameraPropertyValue) -> Result<(), NokhwaError> {
        todo!()
    }
}
