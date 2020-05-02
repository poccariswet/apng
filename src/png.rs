use super::errors::AppResult;
use image::GenericImageView;
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
    let color = img.color();
    let (color_type, bit_depth) = convert_color_png_type(color);

    Ok(PNGImage {
        width: width,
        height: height,
        data: img.raw_pixels(),
        color_type: color_type,
        bit_depth: bit_depth,
    })
}

// make PNGImage from png image decoder
pub fn load_png(filepath: &str) -> AppResult<PNGImage> {
    let file = File::open(filepath).unwrap();
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    // read the frame
    reader.next_frame(&mut buf).unwrap();

    Ok(PNGImage {
        width: info.width,
        height: info.height,
        data: buf,
        color_type: info.color_type,
        bit_depth: info.bit_depth,
    })
}

// cast image::ColorType to png::ColorType
fn convert_color_png_type(ct: image::ColorType) -> (png::ColorType, png::BitDepth) {
    use png::ColorType::*;
    let (ct, bits) = match ct {
        image::ColorType::Gray(bits) => (Grayscale, bits),
        image::ColorType::RGB(bits) => (RGB, bits),
        image::ColorType::Palette(bits) => (Indexed, bits),
        image::ColorType::GrayA(bits) => (GrayscaleAlpha, bits),
        image::ColorType::RGBA(bits) => (RGBA, bits),
        image::ColorType::BGRA(bits) => (RGBA, bits),
        image::ColorType::BGR(bits) => (RGB, bits),
    };
    (ct, png::BitDepth::from_u8(bits).unwrap())
}
