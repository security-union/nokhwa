/*
 * Copyright 2022 l1npengtul <l1npengtul@protonmail.com> / The Nokhwa Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

macro_rules! resolver_platform {
    (
        $( ($name:ident, $feat:literal, $os:literal, $item:path) ),*
    ) => {
        $(
            paste::paste! {
                #[cfg(all(feature = $feat, target_os = $os))]
                pub(crate) fn [< backend_gen_ $name >](index: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    $item::new(index).map(|x| std::boxed::Box::new(x.into()))
                }
                #[cfg(not(all(feature = $feat, target_os = $os)))]
                pub(crate) fn [< backend_gen_ $name >](_: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    return Err(nokhwa_core::error::NokhwaError::GeneralError("no feature".to_string()))
                }
            }
        )*
    };
}

macro_rules! resolver_platform_2 {
    (
        $( ($name:ident, $feat:literal, $os1:literal, $os2:literal, $item:path) ),*
    ) => {
        $(
            paste::paste! {
                #[cfg(all(feature = $feat, target_os = $os1, target_os = $os2))]
                pub(crate) fn [< backend_gen_ $name >](index: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    $item::new(index).map(|x| std::boxed::Box::new(x.into()))
                }
                #[cfg(not(all(feature = $feat, target_os = $os1, target_os = $os2)))]
                pub(crate) fn [< backend_gen_ $name >](_: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    return Err(nokhwa_core::error::NokhwaError::GeneralError("no feature".to_string()))
                }
            }
        )*
    };
}

macro_rules! resolver_cross_platform {
    (
        $( ($name:ident, $feat:literal, $item:path) ),*
    ) => {
        $(
            paste::paste! {
                #[cfg(all(feature = $feat))]
                pub(crate) fn [< backend_gen_ $name >](index: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    $item::new(index).map(|x| std::boxed::Box::new(x.into()))
                }
                #[cfg(not(all(feature = $feat)))]
                pub(crate) fn [< backend_gen_ $name >](_: nokhwa_core::types::CameraIndex) -> Result<Box<dyn nokhwa_core::traits::Backend + nokhwa_core::traits::CaptureTrait>, nokhwa_core::error::NokhwaError> {
                    return Err(nokhwa_core::error::NokhwaError::GeneralError("no feature".to_string()))
                }
            }
        )*
    };
}

resolver_platform!(
    (
        v4l,
        "input-v4l",
        "linux",
        nokhwa_bindings_linux::V4LCaptureDevice
    ),
    (
        msf,
        "input-msmf",
        "windows",
        msmf_backend::MediaFoundationCaptureDevice
    )
);

resolver_platform_2!((
    avf,
    "input-avfoundation",
    "ios",
    "macos",
    avfoundation::AVFoundationCaptureDevice
));

resolver_cross_platform!(
    (opencv, "input-opencv", opencv_backend::OpenCvCaptureDevice) // TODO: wasm
);

#[cfg(all(feature = "input-v4l", target_os = "linux"))]
#[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-v4l")))]
pub use v4l2_backend::V4L2CaptureDevice;
#[cfg(any(
    all(feature = "input-msmf", target_os = "windows"),
    all(feature = "docs-only", feature = "docs-nolink", feature = "input-msmf")
))]
mod msmf_backend;
#[cfg(any(
    all(feature = "input-msmf", target_os = "windows"),
    all(feature = "docs-only", feature = "docs-nolink", feature = "input-msmf")
))]
#[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-msmf")))]
pub use msmf_backend::MediaFoundationCaptureDevice;
#[cfg(any(
    all(
        feature = "input-avfoundation",
        any(target_os = "macos", target_os = "ios")
    ),
    all(
        feature = "docs-only",
        feature = "docs-nolink",
        feature = "input-avfoundation"
    )
))]
mod avfoundation;
#[cfg(any(
    all(
        feature = "input-avfoundation",
        any(target_os = "macos", target_os = "ios")
    ),
    all(
        feature = "docs-only",
        feature = "docs-nolink",
        feature = "input-avfoundation"
    )
))]
#[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-avfoundation")))]
pub use avfoundation::AVFoundationCaptureDevice;
// FIXME: Fix Lifetime Issues
#[cfg(feature = "input-uvc")]
mod uvc_backend;
#[cfg(feature = "input-uvc")]
#[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-uvc")))]
pub use uvc_backend::UVCCaptureDevice;
// #[cfg(feature = "input-gst")]
// mod gst_backend;
// #[cfg(feature = "input-gst")]
// #[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-gst")))]
// pub use gst_backend::GStreamerCaptureDevice;
// #[cfg(feature = "input-jscam")]
// mod browser_backend;
// #[cfg(feature = "input-jscam")]
// #[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-jscam")))]
// pub use browser_backend::BrowserCaptureDevice;
#[cfg(feature = "input-jscam")]
mod browser_camera;
/// A camera that uses `OpenCV` to access IP (rtsp/http) on the local network
// #[cfg(feature = "input-ipcam")]
// #[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-ipcam")))]
// mod network_camera;
// #[cfg(feature = "input-ipcam")]
// #[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-ipcam")))]
// pub use network_camera::NetworkCamera;
#[cfg(feature = "input-opencv")]
mod opencv_backend;
#[cfg(feature = "input-v4l")]
mod v4l2_backend;

#[cfg(feature = "input-opencv")]
#[cfg_attr(feature = "docs-features", doc(cfg(feature = "input-opencv")))]
pub use opencv_backend::OpenCvCaptureDevice;
