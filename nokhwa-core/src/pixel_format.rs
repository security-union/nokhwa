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
use crate::error::NokhwaError;
use crate::types::{
    buf_mjpeg_to_rgb, buf_nv12_to_rgb, buf_yuyv422_to_rgb, color_frame_formats, frame_formats,
    mjpeg_to_rgb, nv12_to_rgb, yuyv422_to_rgb, FrameFormat, Resolution,
};
use image::{Luma, LumaA, Pixel, Rgb, Rgba};
use std::fmt::{format, Debug};

/// Trait that has methods to convert raw data from the webcam to a proper raw image.
pub trait FormatDecoder: Clone + Sized + Send + Sync {
    type Output: Pixel<Subpixel = u8>;
    const FORMATS: &'static [FrameFormat];

    /// Allocates and returns a `Vec`
    /// # Errors
    /// If the data is malformed, or the source [`FrameFormat`] is incompatible, this will error.
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError>;

    /// Writes to a user provided buffer.
    /// # Errors
    /// If the data is malformed, the source [`FrameFormat`] is incompatible, or the user-alloted buffer is not large enough, this will error.
    fn write_output_buffer(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError>;
}

/// A Zero-Size-Type that contains the definition to convert a given image stream to an RGB888 in the [`Buffer`](crate::buffer::Buffer)'s [`.decode_image()`](crate::buffer::Buffer::decode_image)
///
/// ```.ignore
/// use image::{ImageBuffer, Rgb};
/// let image: ImageBuffer<Rgb<u8>, Vec<u8>> = buffer.to_image::<RgbFormat>();
/// ```
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RgbFormat;

impl FormatDecoder for RgbFormat {
    type Output = Rgb<u8>;
    const FORMATS: &'static [FrameFormat] = color_frame_formats();

    #[inline]
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => mjpeg_to_rgb(data, false),
            FrameFormat::YUYV => yuyv422_to_rgb(data, false),
            FrameFormat::GRAY => Ok(data
                .iter()
                .flat_map(|x| {
                    let pxv = *x;
                    [pxv, pxv, pxv]
                })
                .collect()),
            FrameFormat::RAWRGB => Ok(data.to_vec()),
            FrameFormat::NV12 => nv12_to_rgb(resolution, data, false),
            FrameFormat::BGRA => {
                let mut rgb = vec![0u8; data.len()];
                data.chunks_exact(4).enumerate().for_each(|(idx, px)| {
                    let index = idx * 3;
                    rgb[index] = px[2];
                    rgb[index + 1] = px[1];
                    rgb[index + 2] = px[0];
                });
                Ok(rgb)
            }
        }
    }

    #[inline]
    fn write_output_buffer(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => buf_mjpeg_to_rgb(data, dest, false),
            FrameFormat::YUYV => buf_yuyv422_to_rgb(data, dest, false),
            FrameFormat::GRAY => {
                if dest.len() != data.len() * 3 {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "Luma => RGB".to_string(),
                        error: "Bad buffer length".to_string(),
                    });
                }

                data.iter().enumerate().for_each(|(idx, pixel_value)| {
                    let index = idx * 3;
                    dest[index] = *pixel_value;
                    dest[index + 1] = *pixel_value;
                    dest[index + 2] = *pixel_value;
                });
                Ok(())
            }
            FrameFormat::RAWRGB => {
                dest.copy_from_slice(data);
                Ok(())
            }
            FrameFormat::NV12 => buf_nv12_to_rgb(resolution, data, dest, false),
            FrameFormat::BGRA => {
                if dest.len() != data.len() / 4 * 3 {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "BGRA => RGB".to_string(),
                        error: "Bad buffer length".to_string(),
                    });
                }

                data.chunks_exact(4).enumerate().for_each(|(idx, px)| {
                    let index = idx * 3;
                    dest[index] = px[2];
                    dest[index + 1] = px[1];
                    dest[index + 2] = px[0];
                });
                Ok(())
            }
        }
    }
}

/// A Zero-Size-Type that contains the definition to convert a given image stream to an RGBA8888 in the [`Buffer`](crate::buffer::Buffer)'s [`.decode_image()`](crate::buffer::Buffer::decode_image)
///
/// ```.ignore
/// use image::{ImageBuffer, Rgba};
/// let image: ImageBuffer<Rgba<u8>, Vec<u8>> = buffer.to_image::<RgbAFormat>();
/// ```
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RgbAFormat;

impl FormatDecoder for RgbAFormat {
    type Output = Rgba<u8>;

    const FORMATS: &'static [FrameFormat] = color_frame_formats();

    #[inline]
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => mjpeg_to_rgb(data, true),
            FrameFormat::YUYV => yuyv422_to_rgb(data, true),
            FrameFormat::GRAY => Ok(data
                .iter()
                .flat_map(|x| {
                    let pxv = *x;
                    [pxv, pxv, pxv, 255]
                })
                .collect()),
            FrameFormat::RAWRGB => Ok(data
                .chunks_exact(3)
                .flat_map(|x| [x[0], x[1], x[2], 255])
                .collect()),
            FrameFormat::NV12 => nv12_to_rgb(resolution, data, true),
            FrameFormat::BGRA => {
                let mut rgba = vec![0u8; data.len()];
                data.chunks_exact(4).enumerate().for_each(|(idx, px)| {
                    let index = idx * 4;
                    rgba[index] = px[2];
                    rgba[index + 1] = px[1];
                    rgba[index + 2] = px[0];
                    rgba[index + 3] = px[3];
                });
                Ok(rgba)
            }
        }
    }

    #[inline]
    fn write_output_buffer(
        fcc: FrameFormat,
        resolution: Resolution,

        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => buf_mjpeg_to_rgb(data, dest, true),
            FrameFormat::YUYV => buf_yuyv422_to_rgb(data, dest, true),
            FrameFormat::GRAY => {
                if dest.len() != data.len() * 4 {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "Luma => RGBA".to_string(),
                        error: "Bad buffer length".to_string(),
                    });
                }

                data.iter().enumerate().for_each(|(idx, pixel_value)| {
                    let index = idx * 4;
                    dest[index] = *pixel_value;
                    dest[index + 1] = *pixel_value;
                    dest[index + 2] = *pixel_value;
                    dest[index + 3] = 255;
                });
                Ok(())
            }
            FrameFormat::RAWRGB => {
                data.chunks_exact(3).enumerate().for_each(|(idx, px)| {
                    let index = idx * 4;
                    dest[index] = px[0];
                    dest[index + 1] = px[1];
                    dest[index + 2] = px[2];
                    dest[index + 3] = 255;
                });
                Ok(())
            }
            FrameFormat::NV12 => buf_nv12_to_rgb(resolution, data, dest, true),
            FrameFormat::BGRA => {
                if dest.len() != data.len() {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "BGRA => RGBA".to_string(),
                        error: "Bad buffer length".to_string(),
                    });
                }

                data.chunks_exact(4).enumerate().for_each(|(idx, px)| {
                    let index = idx * 4;
                    dest[index] = px[2];
                    dest[index + 1] = px[1];
                    dest[index + 2] = px[0];
                    dest[index + 3] = px[3];
                });
                Ok(())
            }
        }
    }
}

/// A Zero-Size-Type that contains the definition to convert a given image stream to an Luma8(Grayscale 8-bit) in the [`Buffer`](crate::buffer::Buffer)'s [`.decode_image()`](crate::buffer::Buffer::decode_image)
///
/// ```.ignore
/// use image::{ImageBuffer, Luma};
/// let image: ImageBuffer<Luma<u8>, Vec<u8>> = buffer.to_image::<LumaFormat>();
/// ```
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LumaFormat;

impl FormatDecoder for LumaFormat {
    type Output = Luma<u8>;

    const FORMATS: &'static [FrameFormat] = frame_formats();

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => Ok(mjpeg_to_rgb(data, false)?
                .as_slice()
                .chunks_exact(3)
                .map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    (avg / 3) as u8
                })
                .collect()),
            FrameFormat::YUYV => Ok(yuyv422_to_rgb(data, false)?
                .as_slice()
                .chunks_exact(3)
                .map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    (avg / 3) as u8
                })
                .collect()),
            FrameFormat::NV12 => Ok(nv12_to_rgb(resolution, data, false)?
                .as_slice()
                .chunks_exact(3)
                .map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    (avg / 3) as u8
                })
                .collect()),
            FrameFormat::GRAY => Ok(data.to_vec()),
            FrameFormat::RAWRGB => Ok(data
                .chunks(3)
                .map(|px| ((i32::from(px[0]) + i32::from(px[1]) + i32::from(px[2])) / 3) as u8)
                .collect()),
            FrameFormat::BGRA => Ok(data
                .chunks_exact(4)
                .map(|px| ((i32::from(px[0]) + i32::from(px[1]) + i32::from(px[2])) / 3) as u8)
                .collect()),
        }
    }

    #[inline]
    fn write_output_buffer(
        fcc: FrameFormat,
        _resolution: Resolution,
        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError> {
        match fcc {
            // TODO: implement!
            FrameFormat::MJPEG | FrameFormat::YUYV | FrameFormat::NV12 => {
                Err(NokhwaError::ProcessFrameError {
                    src: fcc,
                    destination: "Luma => RGB".to_string(),
                    error: "Conversion Error".to_string(),
                })
            }

            FrameFormat::GRAY => {
                data.iter().zip(dest.iter_mut()).for_each(|(pxv, d)| {
                    *d = *pxv;
                });
                Ok(())
            }
            FrameFormat::RAWRGB => Err(NokhwaError::ProcessFrameError {
                src: fcc,
                destination: "RGB => RGB".to_string(),
                error: "Conversion Error".to_string(),
            }),
            FrameFormat::BGRA => {
                data.chunks_exact(4).zip(dest.iter_mut()).for_each(|(px, d)| {
                    *d = ((i32::from(px[0]) + i32::from(px[1]) + i32::from(px[2])) / 3) as u8;
                });
                Ok(())
            }
        }
    }
}

/// A Zero-Size-Type that contains the definition to convert a given image stream to an LumaA8(Grayscale 8-bit with 8-bit alpha) in the [`Buffer`](crate::buffer::Buffer)'s [`.decode_image()`](crate::buffer::Buffer::decode_image)
///
/// ```.ignore
/// use image::{ImageBuffer, LumaA};
/// let image: ImageBuffer<LumaA<u8>, Vec<u8>> = buffer.to_image::<LumaAFormat>();
/// ```
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LumaAFormat;

impl FormatDecoder for LumaAFormat {
    type Output = LumaA<u8>;

    const FORMATS: &'static [FrameFormat] = frame_formats();

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => Ok(mjpeg_to_rgb(data, false)?
                .as_slice()
                .chunks_exact(3)
                .flat_map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    [(avg / 3) as u8, 255]
                })
                .collect()),
            FrameFormat::YUYV => Ok(yuyv422_to_rgb(data, false)?
                .as_slice()
                .chunks_exact(3)
                .flat_map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    [(avg / 3) as u8, 255]
                })
                .collect()),
            FrameFormat::NV12 => Ok(nv12_to_rgb(resolution, data, false)?
                .as_slice()
                .chunks_exact(3)
                .flat_map(|x| {
                    let mut avg = 0;
                    x.iter().for_each(|v| avg += u16::from(*v));
                    [(avg / 3) as u8, 255]
                })
                .collect()),
            FrameFormat::GRAY => Ok(data.iter().flat_map(|x| [*x, 255]).collect()),
            FrameFormat::RAWRGB => Err(NokhwaError::ProcessFrameError {
                src: fcc,
                destination: "RGB => RGB".to_string(),
                error: "Conversion Error".to_string(),
            }),
            FrameFormat::BGRA => {
                let mut luma_a = vec![0u8; data.len() / 4 * 2];
                data.chunks_exact(4).enumerate().for_each(|(idx, px)| {
                    let index = idx * 2;
                    luma_a[index] = ((i32::from(px[0]) + i32::from(px[1]) + i32::from(px[2])) / 3)
                        as u8;
                    luma_a[index + 1] = px[3];
                });
                Ok(luma_a)
            }
        }
    }

    #[inline]
    fn write_output_buffer(
        fcc: FrameFormat,
        _resolution: Resolution,
        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError> {
        match fcc {
            FrameFormat::MJPEG => {
                // FIXME: implement!
                Err(NokhwaError::ProcessFrameError {
                    src: fcc,
                    destination: "MJPEG => LumaA".to_string(),
                    error: "Conversion Error".to_string(),
                })
            }
            FrameFormat::YUYV => Err(NokhwaError::ProcessFrameError {
                src: fcc,
                destination: "YUYV => LumaA".to_string(),
                error: "Conversion Error".to_string(),
            }),
            FrameFormat::NV12 => Err(NokhwaError::ProcessFrameError {
                src: fcc,
                destination: "NV12 => LumaA".to_string(),
                error: "Conversion Error".to_string(),
            }),
            FrameFormat::GRAY => {
                if dest.len() != data.len() * 2 {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "GRAY8 => LumaA".to_string(),
                        error: "Conversion Error".to_string(),
                    });
                }

                data.iter()
                    .zip(dest.chunks_exact_mut(2))
                    .enumerate()
                    .for_each(|(idx, (pxv, d))| {
                        let index = idx * 2;
                        d[index] = *pxv;
                        d[index + 1] = 255;
                    });
                Ok(())
            }
            FrameFormat::RAWRGB => Err(NokhwaError::ProcessFrameError {
                src: fcc,
                destination: "RGB => RGB".to_string(),
                error: "Conversion Error".to_string(),
            }),
            FrameFormat::BGRA => {
                if dest.len() != data.len() / 4 * 2 {
                    return Err(NokhwaError::ProcessFrameError {
                        src: fcc,
                        destination: "BGRA => LumaA".to_string(),
                        error: "Conversion Error".to_string(),
                    });
                }

                data.chunks_exact(4).zip(dest.chunks_exact_mut(2)).for_each(|(px, d)| {
                    d[0] = ((i32::from(px[0]) + i32::from(px[1]) + i32::from(px[2])) / 3) as u8;
                    d[1] = px[3];
                });
                Ok(())
            }
        }
    }
}



/// let image: ImageBuffer<Rgb<u8>, Vec<u8>> = buffer.to_image::<YuyvFormat>();
/// ```
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct YuyvFormat;

impl FormatDecoder for YuyvFormat {
    type Output = Rgb<u8>;
    const FORMATS: &'static [FrameFormat] = color_frame_formats();

    #[inline]
    fn write_output(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
    ) -> Result<Vec<u8>, NokhwaError> {
        match fcc {
            FrameFormat::YUYV => {
                let i420 = private_convert_yuyv_to_i420(
                    data,
                    resolution.width() as usize,
                    resolution.height() as usize,
                );
                Ok(i420)
            }
            _ => Err(NokhwaError::GeneralError(format!(
                "Invalid FrameFormat: {:?}",
                fcc
            ))),
        }
    }

    #[inline]
    fn write_output_buffer(
        fcc: FrameFormat,
        resolution: Resolution,
        data: &[u8],
        dest: &mut [u8],
    ) -> Result<(), NokhwaError> {
        match fcc {
            FrameFormat::YUYV => {
                convert_yuyv_to_i420_direct(
                    data,
                    dest,
                    resolution.width() as usize,
                    resolution.height() as usize,
                )?;
                Ok(())
            }

            FrameFormat::NV12 => {
                let i420 = nv12_to_i420(data, resolution.width() as usize, resolution.height() as usize);
                // Slice the enough tata to fill the destination buffer, i420 is larger so we need to slice it
                let i420 = &i420[..dest.len()];

                dest.copy_from_slice(&i420);
                Ok(())
            }
            _ => Err(NokhwaError::GeneralError(format!(
                "Invalid FrameFormat: {:?}",
                fcc
            ))),
        }
    }
}

fn private_convert_yuyv_to_i420(yuyv: &[u8], width: usize, height: usize) -> Vec<u8> {
    assert!(
        width % 2 == 0 && height % 2 == 0,
        "Width and height must be even numbers."
    );

    let mut i420 = vec![0u8; width * height + 2 * (width / 2) * (height / 2)];
    let (y_plane, uv_plane) = i420.split_at_mut(width * height);
    let (u_plane, v_plane) = uv_plane.split_at_mut(uv_plane.len() / 2);

    for y in 0..height {
        for x in (0..width).step_by(2) {
            let base_index = (y * width + x) * 2;
            let y0 = yuyv[base_index];
            let u = yuyv[base_index + 1];
            let y1 = yuyv[base_index + 2];
            let v = yuyv[base_index + 3];

            y_plane[y * width + x] = y0;
            y_plane[y * width + x + 1] = y1;

            if y % 2 == 0 {
                u_plane[y / 2 * (width / 2) + x / 2] = u;
                v_plane[y / 2 * (width / 2) + x / 2] = v;
            }
        }
    }

    i420
}

fn convert_yuyv_to_i420_direct(
    yuyv: &[u8],
    dest: &mut [u8],
    width: usize,
    height: usize,
) -> Result<(), NokhwaError> {
    // Ensure the destination buffer is large enough
    if dest.len() < width * height + 2 * (width / 2) * (height / 2) {
        return Err(NokhwaError::GeneralError(
            "Destination buffer is too small".into(),
        ));
    }

    // Split the destination buffer into Y, U, and V planes
    let (y_plane, uv_plane) = dest.split_at_mut(width * height);
    let (u_plane, v_plane) = uv_plane.split_at_mut(uv_plane.len() / 2);

    // Convert YUYV to I420
    for y in 0..height {
        for x in (0..width).step_by(2) {
            let base_index = (y * width + x) * 2;
            let y0 = yuyv[base_index];
            let u = yuyv[base_index + 1];
            let y1 = yuyv[base_index + 2];
            let v = yuyv[base_index + 3];

            y_plane[y * width + x] = y0;
            y_plane[y * width + x + 1] = y1;

            if y % 2 == 0 {
                u_plane[y / 2 * (width / 2) + x / 2] = u;
                v_plane[y / 2 * (width / 2) + x / 2] = v;
            }
        }
    }

    Ok(())
}

fn nv12_to_i420(nv12: &[u8], width: usize, height: usize) -> Vec<u8> {
    let y_plane_size = width * height; // Y plane size
    let uv_plane_size = y_plane_size / 2; // UV plane size
    let single_frame_size = y_plane_size + uv_plane_size;

    // Debugging information

    // Validate buffer size
    // assert_eq!(
    //     nv12.len() % single_frame_size,
    //     0,
    //     "NV12 buffer size is not a multiple of the single frame size"
    // );
    

    // Extract the first frame if the buffer contains multiple frames
    let valid_nv12 = &nv12[..single_frame_size];

    // Allocate space for I420 (Y + U + V planes)
    let mut i420 = vec![0u8; single_frame_size];

    // Copy the Y plane
    i420[..y_plane_size].copy_from_slice(&valid_nv12[..y_plane_size]);

    // Extract the UV plane (interleaved)
    let uv_plane = &valid_nv12[y_plane_size..];

    // Write U and V planes directly to the I420 buffer using indices
    for (i, chunk) in uv_plane.chunks(2).enumerate() {
        let u_index = y_plane_size + i; // Start of U plane
        let v_index = y_plane_size + uv_plane_size / 2 + i; // Start of V plane

        i420[u_index] = chunk[0]; // U value
        i420[v_index] = chunk[1]; // V value
    }

    i420
}
