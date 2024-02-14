/*!
apng is animated png encoder for Rust, and made in pure Rust.

<img src="https://raw.githubusercontent.com/poccariswet/apng/master/examples/_rust_logo/out.png" width="250">

# Example

```no_run
use apng::{Encoder, Frame, PNGImage};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let files = vec![
        "rust_logo1.png",
        "rust_logo2.png",
        "rust_logo3.png",
        "rust_logo4.png",
        "rust_logo5.png",
        "rust_logo6.png",
    ];

    let mut png_images: Vec<PNGImage> = Vec::new();
    for f in files.iter() {
        png_images.push(apng::load_png(f).unwrap());
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
```


# Feature Flags

- `png`: re-exports the types from `png` crate

*/

mod apng;
pub mod errors;
mod png;

pub use crate::apng::*;
pub use crate::png::*;

#[cfg(feature = "png")]
pub use png as image_png;
