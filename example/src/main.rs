use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

fn main() {
    let files = vec![
        "sample/rust_logo1.png",
        "sample/rust_logo2.png",
        "sample/rust_logo3.png",
        "sample/rust_logo4.png",
        "sample/rust_logo5.png",
        "sample/rust_logo6.png",
    ];

    let mut png_images: Vec<PNGImage> = Vec::new();
    /* png file path
    for f in files.iter() {
        png_images.push(apng::load_png(f).unwrap());
    }
    */

    for f in files.iter() {
        let mut file = File::open(f).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let img = image::load_from_memory(&buffer).unwrap();
        png_images.push(apng::load_dynamic_image(img).unwrap());
    }

    let path = Path::new(r"sample/out.png");
    let mut out = BufWriter::new(File::create(path).unwrap());

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();
    let frame = Frame {
        delay_num: Some(1),
        delay_den: Some(2),
        ..Default::default()
    };

    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_n) => println!("success"),
        Err(err) => eprintln!("{}", err),
    }
}
