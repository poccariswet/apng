# apng

[![apng at crates.io](https://img.shields.io/crates/v/apng.svg)](https://crates.io/crates/apng)
[![apng at docs.rs](https://docs.rs/apng/badge.svg)](https://docs.rs/apng)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/poccariswet/apng/master/LICENSE?token=AF4FJMPRTUTCG2DAVLVTRVS5U7UJI)
![apng at GitHub Actions](https://github.com/poccariswet/apng/workflows/Rust/badge.svg?branch=master)

apng is animated png encoder for Rust, and made in pure Rust.

<img src="https://raw.githubusercontent.com/poccariswet/apng/master/examples/_rust_logo/out.png" width="250">

## Example usage

```rust
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

### Customize each frame speed

<img src="https://raw.githubusercontent.com/poccariswet/apng/master/examples/_rust_logo/out.png" width="250">

```rust
    let mut i = 1;
    for image in png_images.iter() {
        i += 1;
        let frame = Frame {
            delay_num: Some(1),
            delay_den: Some(i), // 2, 3, 4, 5, 6, 7
            ..Default::default()
        };
        encoder.write_frame(image, frame).unwrap();
    }
```

Sample code is [here](https://github.com/poccariswet/apng/tree/master/examples/each_frame_speed).

## License

[MIT](https://github.com/poccariswet/apng/blob/master/LICENSE)

## See also

- [Medium article about apng](https://medium.com/@poccariswet/how-i-developed-apng-library-for-rust-98d366f1195b)

## Reference

- [https://developer.mozilla.org/ja/docs/Animated_PNG_graphics](https://web.archive.org/web/20210420033916/https://developer.mozilla.org/ja/docs/Animated_PNG_graphics) ([English](https://web.archive.org/web/20210506203924/https://developer.mozilla.org/en-US/docs/Mozilla/Tech/APNG))
- [https://www.w3.org/TR/PNG/](https://www.w3.org/TR/PNG/)
