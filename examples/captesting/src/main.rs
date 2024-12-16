use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

fn main() {
    // Read index from cli first arg if it fails show usage
    if std::env::args().count() != 2 {
        println!("Usage: captesting <camera_index>");
        std::process::exit(1);
    }
    let index = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();

    let index: CameraIndex = CameraIndex::Index(index);
    let requested: RequestedFormat<'_> =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    let mut camera = Camera::new(index, requested).unwrap();
    println!("{}", camera.camera_format());
    camera.open_stream().unwrap();
    let frame = camera.frame().unwrap();
    camera.stop_stream().unwrap();
    let decoded = frame.decode_image::<RgbFormat>().unwrap();
    decoded
        .save_with_format("turtle.jpeg", image::ImageFormat::Jpeg)
        .unwrap()
}
