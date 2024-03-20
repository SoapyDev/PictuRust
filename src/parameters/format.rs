use std::fmt::{Display, Formatter};

use image::{ImageError, ImageFormat};

use crate::picture::Picture;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Png,
    Jpeg,
    Tiff,
    Webp,
    Avif,
    None,
}

impl Format {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "png" => Self::Png,
            "jpeg" => Self::Jpeg,
            "tiff" => Self::Tiff,
            "webp" => Self::Webp,
            "avif" => Self::Avif,
            _ => Self::None,
        }
    }

    pub fn save_img(&self, img: &mut Picture) {
        output_path_exists(img);
        let _ = match self {
            Self::Png => save_format(img, ImageFormat::Png),
            Self::Jpeg => save_format(img, ImageFormat::Jpeg),
            Self::Tiff => save_format(img, ImageFormat::Tiff),
            Self::Webp => save_format(img, ImageFormat::WebP),
            Self::Avif => save_format(img, ImageFormat::Avif),
            Self::None => img.image.save(&img.output_path),
        };
    }
}

impl Display for Format{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self {
           Format::Png => {write!(f, "png")}
           Format::Jpeg => {write!(f, "jpeg")}
           Format::Tiff => {write!(f, "tiff")}
           Format::Webp => {write!(f, "webp")}
           Format::Avif => {write!(f, "avif")}
           Format::None => {write!(f, "")}
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
        .save_with_format(&img.output_path, format)
}
