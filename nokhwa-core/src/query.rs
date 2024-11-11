use crate::error::NokhwaError;
use crate::types::CameraInfo;

pub trait Query {
    fn query() -> Result<Vec<CameraInfo>, NokhwaError>;
}

pub trait AsyncQuery {
    async fn query() -> Result<Vec<CameraInfo>, NokhwaError>;
}
