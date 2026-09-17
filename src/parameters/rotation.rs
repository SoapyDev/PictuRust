use image::DynamicImage;

use crate::picture::Picture;

use super::parameters::Parameters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
    None,
}

impl Rotation {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "90" => Self::Rotate90,
            "180" => Self::Rotate180,
            "270" => Self::Rotate270,
            _ => Self::None,
        }
    }

    pub fn alter_img(&self, params: &Parameters, img: &mut DynamicImage) {
        rotate(img, *self);
        flip_vertically(params.flip_vertical, img);
        flip_horizontally(params.flip_horizontal, img);
    }
}

fn rotate(img: &mut DynamicImage, rotation: Rotation) {
    match rotation {
        Rotation::Rotate90 => *img = img.rotate90(),
        Rotation::Rotate180 => *img = img.rotate180(),
        Rotation::Rotate270 => *img = img.rotate270(),
        Rotation::None => (),
    }
}

fn flip_vertically(flip_v: bool, img: &mut DynamicImage) {
    if flip_v {
        *img = img.flipv();
    }
}
fn flip_horizontally(flip_h: bool, img: &mut DynamicImage) {
    if flip_h {
        *img = img.fliph();
    }
}

pub fn set_initial_rotation(code: Option<u32>, picture: &mut Picture) {
    match code {
        Some(6) => {
            picture.image = picture.image.rotate90();
        }
        Some(3) => {
            picture.image = picture.image.rotate180();
        }
        Some(8) => {
            picture.image = picture.image.rotate270();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::format::Format;
    use crate::parameters::resizetype::ResizeType;
    use image::{DynamicImage, GenericImageView, RgbImage};
    use std::path::PathBuf;

    fn params(rotation: Rotation, flip_h: bool, flip_v: bool) -> Parameters {
        Parameters {
            input_dir: PathBuf::new(),
            recursive: false,
            output_dir: PathBuf::new(),
            width: None,
            height: None,
            resize_type: ResizeType::None,
            filter: image::imageops::FilterType::Lanczos3,
            format: Format::None,
            quality: 75.0,
            speed: 7,
            rotation,
            flip_horizontal: flip_h,
            flip_vertical: flip_v,
        }
    }

    fn asymmetric_image() -> DynamicImage {
        // A non-square image so rotation actually changes the dimensions.
        DynamicImage::ImageRgb8(RgbImage::new(4, 2))
    }

    #[test]
    fn parses_known_rotations() {
        assert_eq!(Rotation::new("90"), Rotation::Rotate90);
        assert_eq!(Rotation::new("180"), Rotation::Rotate180);
        assert_eq!(Rotation::new("270"), Rotation::Rotate270);
        assert_eq!(Rotation::new("bogus"), Rotation::None);
    }

    #[test]
    fn rotate_90_swaps_dimensions() {
        let mut img = asymmetric_image();
        let p = params(Rotation::Rotate90, false, false);
        p.rotation.alter_img(&p, &mut img);
        assert_eq!(img.dimensions(), (2, 4));
    }

    #[test]
    fn rotate_180_keeps_dimensions() {
        let mut img = asymmetric_image();
        let p = params(Rotation::Rotate180, false, false);
        p.rotation.alter_img(&p, &mut img);
        assert_eq!(img.dimensions(), (4, 2));
    }

    #[test]
    fn no_rotation_or_flip_is_a_no_op() {
        let mut img = asymmetric_image();
        let original = img.clone();
        let p = params(Rotation::None, false, false);
        p.rotation.alter_img(&p, &mut img);
        assert_eq!(img.to_rgb8(), original.to_rgb8());
    }

    #[test]
    fn flip_horizontal_and_vertical_preserve_dimensions() {
        let mut img = asymmetric_image();
        let p = params(Rotation::None, true, true);
        p.rotation.alter_img(&p, &mut img);
        assert_eq!(img.dimensions(), (4, 2));
    }
}
