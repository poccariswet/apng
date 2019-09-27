use apng;
use std::fs::File;

fn main() {
    println!("let's go apng!");

    let files = vec![
        "sample/ouija1.png",
        "sample/ouija2.png",
        "sample/ouija3.png",
        "sample/ouija4.png",
    ];

    let mut pngs: Vec<PNGImage> = Vec::new();
    for f in files.iter() {
        pngs.push(load_png(f).unwrap());
    }

    let mut out = File::create("sample/test.png").unwrap();
    let mut apng = APNG { images: pngs };
    let mut encoder = Encoder::new(&mut out, apng.create_config(0).unwrap()).unwrap();
    let _frame = Frame {
        delay_num: Some(1),
        delay_den: Some(5),
        ..Default::default()
    };
    let err = encoder.encode_all(apng);
    println!("{:?}", err)
}
