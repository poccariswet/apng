use super::errors::AppResult;
use image::{DynamicImage, GenericImageView};
use png::BitDepth;
use std::fs::File;

#[derive(Clone, Debug, PartialEq)]
pub struct PNGImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub color_type: png::ColorType,
    pub bit_depth: png::BitDepth,
}

// make PNGImage from image::DynamicImage.
pub fn load_dynamic_image(img: image::DynamicImage) -> AppResult<PNGImage> {
    let (width, height) = img.dimensions();
    let (data, color_type, bit_depth) = get_raw_buffer_dynamic_image(img);

    Ok(PNGImage {
        width: width,
        height: height,
        data,
        color_type,
        bit_depth,
    })
}

// make PNGImage from png image decoder
pub fn load_png(filepath: &str) -> AppResult<PNGImage> {
    let file = File::open(filepath).unwrap();
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];

    // read the frame
    let info = reader.next_frame(&mut buf).unwrap();

    Ok(PNGImage {
        width: info.width,
        height: info.height,
        data: buf,
        color_type: info.color_type,
        bit_depth: info.bit_depth,
    })
}

/// Safely convert a Vec<u16> to a Vec<u8>
fn vec16_to_vec8(input: Vec<u16>) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 2);
    for nb in input {
        output.extend(&nb.to_le_bytes());
    }
    output
}

/// convert an [`image::DynamicImage`] into a raw buffer, a [`png::ColorType`] and a [`png::BitDepth`]
fn get_raw_buffer_dynamic_image(
    dynamic_image: DynamicImage,
) -> (Vec<u8>, png::ColorType, png::BitDepth) {
    use png::ColorType::*;

    match dynamic_image {
        DynamicImage::ImageRgb8(image) => (image.into_raw(), Rgb, BitDepth::Eight),
        DynamicImage::ImageLuma8(image) => (image.into_raw(), Grayscale, BitDepth::Eight),
        DynamicImage::ImageLumaA8(image) => (image.into_raw(), GrayscaleAlpha, BitDepth::Eight),
        DynamicImage::ImageRgba8(image) => (image.into_raw(), Rgba, BitDepth::Eight),
        DynamicImage::ImageBgr8(image) => (
            DynamicImage::ImageBgr8(image).into_rgb8().into_raw(),
            Rgb,
            BitDepth::Eight,
        ),
        DynamicImage::ImageBgra8(image) => (
            DynamicImage::ImageBgra8(image).into_rgb8().into_raw(),
            Rgb,
            BitDepth::Eight,
        ),
        DynamicImage::ImageLuma16(image) => (
            vec16_to_vec8(image.into_raw()),
            Grayscale,
            BitDepth::Sixteen,
        ),
        DynamicImage::ImageLumaA16(image) => (
            vec16_to_vec8(image.into_raw()),
            GrayscaleAlpha,
            BitDepth::Sixteen,
        ),
        DynamicImage::ImageRgb16(image) => {
            (vec16_to_vec8(image.into_raw()), Rgb, BitDepth::Sixteen)
        }
        DynamicImage::ImageRgba16(image) => {
            (vec16_to_vec8(image.into_raw()), Rgba, BitDepth::Sixteen)
        }
    }
}
