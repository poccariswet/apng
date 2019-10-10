use super::errors::{APNGError, APNGResult, AppResult};
use byteorder::{BigEndian, WriteBytesExt};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use flate2::Crc;
use std::fs::File;
use std::io::{self, Write};
use std::mem;

#[derive(Clone, Debug, PartialEq)]
pub struct APNG {
    pub images: Vec<PNGImage>, // The successive png images
}

impl APNG {
    pub fn create_config(&mut self, plays: u32) -> APNGResult<Config> {
        if self.images.len() == 0 {
            return Err(APNGError::ImagesNotFound);
        }
        let image = self.images[0].clone();
        Ok(Config {
            width: image.width,
            height: image.height,
            num_frames: self.images.len() as u32,
            num_plays: plays,
            color: image.color_type,
            depth: image.bit_depth,
            filter: png::FilterType::NoFilter, //default
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PNGImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub color_type: png::ColorType,
    pub bit_depth: png::BitDepth,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    // number of frames
    pub num_frames: u32,
    // count of loop, 0 is infinite looping
    pub num_plays: u32,
    pub color: png::ColorType,
    pub depth: png::BitDepth,
    pub filter: png::FilterType,
}

impl Config {
    // Returns the bits per pixel
    pub fn bytes_per_pixel(&self) -> usize {
        self.color.samples() * self.depth as usize
    }

    // Returns the number of bytes needed for one deinterlaced row
    pub fn raw_row_length(&self) -> usize {
        let bits = self.width as usize * self.color.samples() * self.depth as usize;
        let extra = bits % 8;
        bits / 8
            + match extra {
                0 => 0,
                _ => 1,
            }
            + 1 // filter method
    }
}

#[derive(Debug, PartialEq)]
pub struct Encoder<'a, W: io::Write> {
    config: Config,
    w: &'a mut W,
    seq_num: u32,
}

impl<'a, W: io::Write> Encoder<'a, W> {
    pub fn new(writer: &'a mut W, config: Config) -> APNGResult<Self> {
        let mut e = Encoder {
            config: config,
            w: writer,
            seq_num: 0,
        };
        Self::write_png_header(&mut e)?;
        Self::write_ihdr(&mut e)?;
        Self::write_ac_tl(&mut e)?;
        Ok(e)
    }

    // all png images encode to apng
    pub fn encode_all(&mut self, apng: APNG, frame: Option<&Frame>) -> APNGResult<()> {
        let mut i = 0;
        for v in apng.images.iter() {
            if i == 0 {
                Self::write_fc_tl(self, frame)?;
                Self::write_idats(self, &v.data)?;
            } else {
                Self::write_fc_tl(self, frame)?;
                Self::write_fd_at(self, &v.data)?;
            }
            i += 1;
        }
        Self::write_iend(self)?;
        Ok(())
    }

    fn write_png_header(&mut self) -> APNGResult<()> {
        self.w.write_all(b"\x89PNG\r\n\x1a\n")?;
        Ok(())
    }

    fn write_iend(&mut self) -> APNGResult<()> {
        self.write_chunk(&[], *b"IEND")
    }

    fn write_ihdr(&mut self) -> APNGResult<()> {
        let mut buf = vec![];
        buf.write_u32::<BigEndian>(self.config.width)?;
        buf.write_u32::<BigEndian>(self.config.height)?;
        buf.write_all(&[self.config.depth as u8, self.config.color as u8, 0, 0, 0])?;
        self.write_chunk(&buf, *b"IHDR")
    }

    fn write_ac_tl(&mut self) -> APNGResult<()> {
        let mut buf = vec![];
        buf.write_u32::<BigEndian>(self.config.num_frames)?;
        buf.write_u32::<BigEndian>(self.config.num_plays)?;
        self.write_chunk(&buf, *b"acTL")
    }

    fn write_fc_tl(&mut self, frame: Option<&Frame>) -> APNGResult<()> {
        let mut buf = vec![];
        buf.write_u32::<BigEndian>(self.seq_num)?;
        buf.write_u32::<BigEndian>(frame.and_then(|f| f.width).unwrap_or(self.config.width))?;
        buf.write_u32::<BigEndian>(frame.and_then(|f| f.height).unwrap_or(self.config.height))?;
        buf.write_u32::<BigEndian>(frame.and_then(|f| f.offset_x).unwrap_or(0))?;
        buf.write_u32::<BigEndian>(frame.and_then(|f| f.offset_y).unwrap_or(0))?;
        buf.write_u16::<BigEndian>(frame.and_then(|f| f.delay_num).unwrap_or(1))?;
        buf.write_u16::<BigEndian>(frame.and_then(|f| f.delay_den).unwrap_or(3))?;

        let dis = frame
            .and_then(|f| f.dispose_op)
            .unwrap_or(DisposeOp::ApngDisposeOpNone) as u8;
        let ble = frame
            .and_then(|f| f.blend_op)
            .unwrap_or(BlendOp::ApngBlendOpSource) as u8;
        buf.write_all(&[dis, ble])?;

        self.write_chunk(&buf, *b"fcTL")?;
        self.seq_num += 1;

        Ok(())
    }

    fn write_fd_at(&mut self, data: &[u8]) -> APNGResult<()> {
        let mut buf = vec![];
        buf.write_u32::<BigEndian>(self.seq_num)?;
        self.make_image_buffer(data, &mut buf)?;
        self.write_chunk(&buf, *b"fdAT")?;
        self.seq_num += 1;
        Ok(())
    }

    // Writes the image data.
    fn write_idats(&mut self, data: &[u8]) -> APNGResult<()> {
        let mut buf = vec![];
        self.make_image_buffer(data, &mut buf)?;
        self.write_chunk(&buf, *b"IDAT")?;
        Ok(())
    }

    fn make_image_buffer(&mut self, data: &[u8], buf: &mut Vec<u8>) -> APNGResult<()> {
        let bpp = self.config.bytes_per_pixel();
        let in_len = self.config.raw_row_length() - 1;

        let mut prev = vec![0; in_len];
        let mut current = vec![0; in_len];

        let data_size = in_len * self.config.height as usize;
        if data_size != data.len() {
            return Err(APNGError::WrongDataSize(data_size, data.len()));
        }

        let mut zlib = ZlibEncoder::new(buf, Compression::best());
        let filter_method = self.config.filter;

        for line in data.chunks(in_len) {
            current.copy_from_slice(&line);
            zlib.write_all(&[filter_method as u8])?;
            filter(filter_method, bpp, &prev, &mut current);
            zlib.write_all(&current)?;
            mem::swap(&mut prev, &mut current);
        }

        zlib.finish()?;
        Ok(())
    }

    // write chunk data 4 field
    fn write_chunk(&mut self, c_data: &[u8], c_type: [u8; 4]) -> APNGResult<()> {
        // Header(Length and Type)
        self.w.write_u32::<BigEndian>(c_data.len() as u32)?;
        self.w.write_all(&c_type)?;
        // Data
        self.w.write_all(c_data)?;
        // Footer (CRC)
        let mut crc = Crc::new();
        crc.update(&c_type);
        crc.update(c_data);
        self.w.write_u32::<BigEndian>(crc.sum() as u32)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Frame {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub offset_x: Option<u32>,
    pub offset_y: Option<u32>,
    pub delay_num: Option<u16>,        // numerator of frame delay
    pub delay_den: Option<u16>,        // denominator of framge delay
    pub dispose_op: Option<DisposeOp>, // specifies before rendering the next frame
    pub blend_op: Option<BlendOp>, // specifies whether to blend alpha blend or replace the output buffer
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisposeOp {
    ApngDisposeOpNone = 0,
    ApngDisposeOpBackground = 1,
    ApngDisposeOpPrevious = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlendOp {
    ApngBlendOpSource = 0,
    ApngBlendOpOver = 1,
}

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

fn filter_path(a: u8, b: u8, c: u8) -> u8 {
    let ia = i16::from(a);
    let ib = i16::from(b);
    let ic = i16::from(c);

    let p = ia + ib - ic;

    let pa = (p - ia).abs();
    let pb = (p - ib).abs();
    let pc = (p - ic).abs();

    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

pub fn filter(method: png::FilterType, bpp: usize, previous: &[u8], current: &mut [u8]) {
    use png::FilterType::*;
    assert!(bpp > 0);
    let len = current.len();

    match method {
        NoFilter => (),
        Sub => {
            for i in (bpp..len).rev() {
                current[i] = current[i].wrapping_sub(current[i - bpp]);
            }
        }
        Up => {
            for i in 0..len {
                current[i] = current[i].wrapping_sub(previous[i]);
            }
        }
        Avg => {
            for i in (bpp..len).rev() {
                current[i] =
                    current[i].wrapping_sub(current[i - bpp].wrapping_add(previous[i]) / 2);
            }

            for i in 0..bpp {
                current[i] = current[i].wrapping_sub(previous[i] / 2);
            }
        }
        Paeth => {
            for i in (bpp..len).rev() {
                current[i] = current[i].wrapping_sub(filter_path(
                    current[i - bpp],
                    previous[i],
                    previous[i - bpp],
                ));
            }

            for i in 0..bpp {
                current[i] = current[i].wrapping_sub(filter_path(0, previous[i], 0));
            }
        }
    }
}
