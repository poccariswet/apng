use apng;
use apng::Encoder;
use apng::{Frame, PNGImage, APNG};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let files = vec![
        "sample/rust_logo1.png",
        "sample/rust_logo3.png",
        "sample/rust_logo3.png",
        "sample/rust_logo4.png",
        "sample/rust_logo5.png",
        "sample/rust_logo6.png",
    ];

    let mut pngs: Vec<PNGImage> = Vec::new();
    for f in files.iter() {
        pngs.push(apng::load_png(f).unwrap());
    }

    let path = Path::new(r"sample/out.png");
    let mut out = BufWriter::new(File::create(path).unwrap());

    let mut apng = APNG { images: pngs };
    let config = apng.create_config(0).unwrap();
    let mut encoder = Encoder::new(&mut out, config).unwrap();
    let frame = Frame {
        delay_num: Some(1),
        delay_den: Some(2),
        ..Default::default()
    };
    let err = encoder.encode_all(apng, Some(&frame));
    println!("{:?}", err)
}
