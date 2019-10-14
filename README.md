# apng

[![apng at crates.io](https://img.shields.io/crates/v/apng.svg)](https://crates.io/crates/apng)
[![apng at docs.rs](https://docs.rs/apng/badge.svg)](https://docs.rs/apng)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/poccariswet/apng/master/LICENSE?token=AF4FJMPRTUTCG2DAVLVTRVS5U7UJI)

apng is animation png encoder for rust, and made in pure rust.

![animation rust logo](https://github.com/poccariswet/apng/blob/master/example/sample/out.png)

## Example usage

```rust
fn main() {
    let files = vec![
        "sample/rust_logo1.png",
        "sample/rust_logo2.png",
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
```

## License

[here](https://github.com/poccariswet/apng/blob/master/LICENSE)

## Reference

- [https://developer.mozilla.org/ja/docs/Animated_PNG_graphics](https://developer.mozilla.org/ja/docs/Animated_PNG_graphics)
- [https://www.w3.org/TR/PNG/](https://www.w3.org/TR/PNG/)
