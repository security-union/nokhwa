use crate::camera::{AsyncCamera, Camera};
use crate::error::NokhwaResult;
use crate::types::{CameraIndex, CameraInformation};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Backends {
    Video4Linux2,
    WebWASM,
    AVFoundation,
    MicrosoftMediaFoundation,
    Custom(&'static str)
}

pub trait PlatformTrait {
    const PLATFORM: Backends;
    type Camera: Camera;


    fn block_on_permission(&mut self) -> NokhwaResult<()>;

    fn check_permission_given(&mut self) -> bool;

    fn query(&mut self) -> NokhwaResult<Vec<CameraInformation>>;

    fn open(&mut self, index: &CameraIndex) -> NokhwaResult<Self::Camera>;
}

#[cfg(feature = "async")]
pub trait AsyncPlatformTrait {
    const PLATFORM: Backends;
    type AsyncCamera: AsyncCamera;


    async fn await_permission(&mut self) -> NokhwaResult<()>;

    async fn query_async(&mut self) -> NokhwaResult<Vec<CameraInformation>>;

    async fn open_async (&mut self, index: &CameraIndex) -> NokhwaResult<Self::AsyncCamera>;
}