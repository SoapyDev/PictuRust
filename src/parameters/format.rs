use image::{ImageError, ImageFormat};
use ravif::*;
use rgb::*;

use crate::picture::Picture;

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

    pub fn reformat_image(
        &self,
        img: &mut Picture,
        quality: f32,
        speed: u8,
    ) -> Result<(), ImageError> {
        if img.output_path.exists() {
            create_new_output_path(img);
        }
        match self {
            Self::PNG => img
                .image
                .save_with_format(img.output_path.to_owned(), ImageFormat::Png),
            Self::JPEG => img
                .image
                .save_with_format(img.output_path.to_owned(), ImageFormat::Jpeg),
            Self::TIFF => img
                .image
                .save_with_format(img.output_path.to_owned(), ImageFormat::Tiff),
            Self::WEBP => {
                let encoded_img = webp::Encoder::from_image(&img.image)
                    .expect("Failed to create webp encoder")
                    .encode(quality);
                let encoded_img = encoded_img.get(0..).expect("Failed to encode webp");
                _ = std::fs::write(img.output_path.to_owned(), encoded_img);
                Ok(())
            }
            Self::AVIF => {
                let raw = img.image.to_rgba8().into_raw();
                let rgb: &[RGBA8] = raw.as_rgba();
                let avif = Encoder::new()
                    .with_speed(speed)
                    .with_quality(quality)
                    .encode_rgba(Img::new(
                        &rgb,
                        img.image.width().try_into().unwrap(),
                        img.image.height().try_into().unwrap(),
                    ))
                    .expect("Failed to encode avif");
                _ = std::fs::write(img.output_path.to_owned(), avif.avif_file);
                Ok(())
            }
            Self::None => img.image.save(img.output_path.to_owned()),
        }
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
fn create_new_output_path(img: &mut Picture) {
    let mut index = 1;
    loop {
        let mut new_path = img.output_path.clone();
        new_path.set_file_name(format!(
            "{}_{}.{}",
            img.output_path.file_stem().unwrap().to_str().unwrap(),
            index,
            img.output_path.extension().unwrap().to_str().unwrap()
        ));
        if !new_path.exists() {
            img.output_path = new_path;
            break;
        }
        index += 1;
    }
}
