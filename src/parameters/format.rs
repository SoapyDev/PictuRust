use image::{ImageError, ImageFormat};
use ravif::*;
use rgb::*;

use crate::picture::Picture;

use super::parameters::Parameters;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    PNG,
    JPEG,
    TIFF,
    WEBP,
    AVIF,
    None,
}

impl Format {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "png" => Self::PNG,
            "jpeg" => Self::JPEG,
            "tiff" => Self::TIFF,
            "webp" => Self::WEBP,
            "avif" => Self::AVIF,
            _ => Self::None,
        }
    }

    pub fn save_img(&self, img: &mut Picture, params: &Parameters) {
        output_path_exists(img);
        let _ = match self {
            Self::PNG => save_format(img, ImageFormat::Png),
            Self::JPEG => save_format(img, ImageFormat::Jpeg),
            Self::TIFF => save_format(img, ImageFormat::Tiff),
            Self::WEBP => save_webp(img, params),
            Self::AVIF => save_avif(img, params),
            Self::None => img.image.save(img.output_path.to_owned()),
        };
    }

    pub(crate) fn to_string(&self) -> &str {
        match self {
            Self::PNG => "png",
            Self::JPEG => "jpeg",
            Self::TIFF => "tiff",
            Self::WEBP => "webp",
            Self::AVIF => "avif",
            Self::None => "",
        }
    }
}

fn output_path_exists(img: &mut Picture) {
    if img.output_path.exists() {
        create_new_output_path(img);
    }
}
fn create_new_output_path(img: &mut Picture) {
    let mut index = 1;
    let mut new_path = img.output_path.clone();
    loop {
        new_path.set_file_name(format!(
            "{}_{}.{}",
            img.output_path.file_stem().unwrap().to_str().unwrap(),
            index,
            img.output_path.extension().unwrap().to_str().unwrap()
        ));
        if !new_path.exists() {
            img.output_path = new_path;
            return;
        }
        index += 1;
    }
}

fn save_format(img: &mut Picture, format: ImageFormat) -> Result<(), ImageError> {
    img.image
        .save_with_format(img.output_path.to_owned(), format)
}

fn save_webp(img: &mut Picture, params: &Parameters) -> Result<(), ImageError> {
    let encoded_img = into_webp(img, params);
    save_image_with_special_format(img, encoded_img.as_slice());
    Ok(())
}

fn into_webp<'a>(img: &'a mut Picture, params: &'a Parameters) -> Vec<u8> {
    let encoded_img = webp::Encoder::from_image(&img.image)
        .expect("Failed to create webp encoder")
        .encode(params.quality);
    let encoded_img = encoded_img.get(0..).expect("Failed to encode webp");
    encoded_img.to_owned()
}

fn save_avif(img: &mut Picture, params: &Parameters) -> Result<(), ImageError> {
    let avif = into_avif(img, params);
    save_image_with_special_format(img, &avif.avif_file);
    Ok(())
}

fn into_avif(img: &mut Picture, params: &Parameters) -> EncodedImage {
    let width = img.image.width() as usize;
    let height = img.image.height() as usize;
    let avif = Encoder::new()
        .with_speed(params.speed)
        .with_quality(params.quality)
        .encode_rgba(Img::new(to_raw_rgba8(img).as_rgba(), width, height))
        .expect("Failed to encode avif");
    avif
}

fn to_raw_rgba8(img: &mut Picture) -> Vec<u8> {
    img.image.to_rgba8().into_raw()
}

fn save_image_with_special_format(img: &mut Picture, encoded_img: &[u8]) {
    _ = std::fs::write(img.output_path.to_owned(), encoded_img);
}
