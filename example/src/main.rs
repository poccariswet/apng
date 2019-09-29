use apng;
use apng::Encoder;
use apng::{Frame, PNGImage, APNG};
use std::fs::File;

fn main() {
    let files = vec![
        "sample/logo1.png",
        "sample/logo2.png",
        "sample/logo3.png",
        //"sample/logo4.png",
        //"sample/logo5.png",
        //"sample/logo6.png",
        //"sample/logo7.png",
        //"sample/logo8.png",
    ];

    let mut pngs: Vec<PNGImage> = Vec::new();
    for f in files.iter() {
        pngs.push(apng::load_png(f).unwrap());
    }

    let mut out = File::create("sample/out.png").unwrap();
    let mut apng = APNG { images: pngs };
    let mut encoder = Encoder::new(&mut out, apng.create_config(0).unwrap()).unwrap();
    let frame = Frame {
        delay_num: Some(1),
        delay_den: Some(2),
        ..Default::default()
    };
    let err = encoder.encode_all(apng, Some(&frame));
    println!("{:?}", err)
}
