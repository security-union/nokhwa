use crate::utils::Distance;
use crate::{
    frame_format::FrameFormat,
    ranges::Range,
    types::{CameraFormat, FrameRate, Resolution},
};
use std::cmp::Ordering;
use crate::ranges::ValidatableRange;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum ClosestType {
    Resolution,
    FrameRate,
    Both,
    None,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum CustomFormatRequestType {
    HighestFrameRate,
    HighestResolution,
    Closest,
    Exact,
}

/// A helper for choosing a [`CameraFormat`].
/// The use of this is completely optional - for a simpler way try [`crate::camera::Camera::enumerate_formats`].
///
/// The `frame_format` field filters out the [`CameraFormat`]s by [`FrameFormat`].
pub enum FormatRequest {
    /// Pick the closest [`CameraFormat`] to the one requested
    Closest {
        resolution: Option<Range<Resolution>>,
        frame_rate: Option<Range<FrameRate>>,
        frame_format: Vec<FrameFormat>,
    },
    HighestFrameRate {
        frame_rate: Range<FrameRate>,
        frame_format: Vec<FrameFormat>,
    },
    HighestResolution {
        resolution: Range<Resolution>,
        frame_format: Vec<FrameFormat>,
    },
    Exact {
        resolution: Resolution,
        frame_rate: FrameRate,
        frame_format: Vec<FrameFormat>,
    },
}

impl FormatRequest {
    pub fn sort_formats(&self, list_of_formats: &[CameraFormat]) -> Vec<CameraFormat> {
        if list_of_formats.is_empty() {
            return vec![];
        }

        match self {
            FormatRequest::Closest {
                resolution,
                frame_rate,
                frame_format,
            } => {
                let resolution_point = resolution.map(|x| x.preferred());
                let frame_rate_point = frame_rate.map(|x| x.preferred());
                // lets calcuate distance in 3 dimensions (add both resolution and frame_rate together)

                let mut distances = list_of_formats
                    .iter()
                    .filter(|x| frame_format.contains(&x.format()))
                    .map(|fmt| {
                        let frame_rate_distance = match frame_rate_point {
                            Some(f_point) => (fmt.frame_rate() - f_point).approximate_float().unwrap_or(f32::INFINITY).abs(),
                            None => 0_f32,
                        };
                        
                        let resolution_point_distance = match resolution_point {
                            Some(res_pt) => fmt.resolution().distance_from(&res_pt) as f32,
                            None => 0_f32,
                        };
                        
                        (frame_rate_distance + resolution_point_distance, fmt)
                    })
                    .collect::<Vec<(f32, &CameraFormat)>>();
                distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
                distances.into_iter().map(|x| x.1).copied().collect()
            }
            FormatRequest::HighestFrameRate {
                frame_rate,
                frame_format,
            } => {
                let mut formats = list_of_formats
                    .iter()
                    .filter(|x| {
                        frame_format.contains(&x.format()) && frame_rate.validate(&x.frame_rate()).is_ok()
                    })
                    .collect::<Vec<_>>();
                formats.sort();
                formats.into_iter().copied().collect()
            }
            FormatRequest::HighestResolution {
                resolution,
                frame_format,
            } => {
                let mut formats = list_of_formats
                    .iter()
                    .filter(|x| {
                        frame_format.contains(&x.format()) && resolution.validate(&x.resolution()).is_ok()
                    })
                    .collect::<Vec<_>>();
                formats.sort();
                formats.into_iter().copied().collect()
            }
            FormatRequest::Exact {
                resolution,
                frame_rate,
                frame_format,
            } => {
                let mut formats = list_of_formats
                    .iter()
                    .filter(|x| {
                        frame_format.contains(&x.format())
                            && resolution == &x.resolution()
                            && frame_rate == &x.frame_rate()
                    })
                    .collect::<Vec<_>>();
                formats.sort();
                formats.into_iter().copied().collect()
            }
        }
    }

    ///
    #[must_use]
    pub fn resolve(&self, list_of_formats: &[CameraFormat]) -> Option<CameraFormat> {
        if list_of_formats.is_empty() {
            return None;
        }

        Some(self.sort_formats(list_of_formats).remove(0))
    }
}
