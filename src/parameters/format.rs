use image::{ImageError, ImageFormat};

use crate::picture::Picture;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    PNG,
    JPEG,
    TIFF,
    //WEBP,
    //AVIF,
    None,
}

impl Format {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "png" => Self::PNG,
            "jpeg" => Self::JPEG,
            "tiff" => Self::TIFF,
            //      "webp" => Self::WEBP,
            //      "avif" => Self::AVIF,
            _ => Self::None,
        }
    }

    pub fn reformat_image(&self, img: &mut Picture) -> Result<(), ImageError> {
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
            // Self::WEBP => img
            //     .image
            //     .save_with_format(img.output_path.to_owned(), ImageFormat::WebP),
            // Self::AVIF => img
            //     .image
            //     .save_with_format(img.output_path.to_owned(), ImageFormat::Avif),
            Self::None => img.image.save(img.output_path.to_owned()),
        }
    }

    pub(crate) fn to_string(&self) -> &str {
        match self {
            Self::PNG => "png",
            Self::JPEG => "jpeg",
            Self::TIFF => "tiff",
            // Self::WEBP => "webp",
            // Self::AVIF => "avif",
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
