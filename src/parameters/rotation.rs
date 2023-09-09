use image::DynamicImage;

use crate::picture::Picture;

use super::parameters::Parameters;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
    None,
}

impl Rotation {
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
        flip_horizontally(params.flip_horizontal, img)
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
