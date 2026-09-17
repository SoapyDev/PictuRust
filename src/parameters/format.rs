use std::fmt::{Display, Formatter};

use image::{ImageError, ImageFormat};

use crate::picture::Picture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpeg,
    Tiff,
    Webp,
    Avif,
    None,
}

impl Format {
    #[must_use]
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

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Png => {
                write!(f, "png")
            }
            Self::Jpeg => {
                write!(f, "jpeg")
            }
            Self::Tiff => {
                write!(f, "tiff")
            }
            Self::Webp => {
                write!(f, "webp")
            }
            Self::Avif => {
                write!(f, "avif")
            }
            Self::None => {
                write!(f, "")
            }
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

fn save_format(img: &Picture, format: ImageFormat) -> Result<(), ImageError> {
    img.image.save_with_format(&img.output_path, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbImage};

    fn picture(output_path: std::path::PathBuf) -> Picture {
        Picture {
            output_path,
            dimensions: (1, 1),
            image: DynamicImage::ImageRgb8(RgbImage::new(1, 1)),
        }
    }

    #[test]
    fn parses_known_formats_case_insensitively() {
        assert_eq!(Format::new("Png"), Format::Png);
        assert_eq!(Format::new("JPEG"), Format::Jpeg);
        assert_eq!(Format::new("tiff"), Format::Tiff);
        assert_eq!(Format::new("WebP"), Format::Webp);
        assert_eq!(Format::new("avif"), Format::Avif);
        assert_eq!(Format::new("bogus"), Format::None);
    }

    #[test]
    fn displays_as_lowercase_extension() {
        assert_eq!(Format::Png.to_string(), "png");
        assert_eq!(Format::Jpeg.to_string(), "jpeg");
        assert_eq!(Format::Tiff.to_string(), "tiff");
        assert_eq!(Format::Webp.to_string(), "webp");
        assert_eq!(Format::Avif.to_string(), "avif");
        assert_eq!(Format::None.to_string(), "");
    }

    #[test]
    fn avoids_overwriting_an_existing_output_file() {
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().join("photo.png");
        std::fs::write(&existing, b"placeholder").unwrap();

        let mut img = picture(existing);
        output_path_exists(&mut img);

        assert_eq!(img.output_path, dir.path().join("photo_1.png"));
    }

    #[test]
    fn keeps_bumping_suffix_until_a_free_name_is_found() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("photo.png"), b"placeholder").unwrap();
        std::fs::write(dir.path().join("photo_1.png"), b"placeholder").unwrap();

        let mut img = picture(dir.path().join("photo.png"));
        output_path_exists(&mut img);

        assert_eq!(img.output_path, dir.path().join("photo_2.png"));
    }
}
