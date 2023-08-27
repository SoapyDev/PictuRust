use image::DynamicImage;

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

    pub fn rotate_image(&self, flip_v: bool, flip_h: bool, img: &mut DynamicImage) {
        match self {
            Self::Rotate90 => *img = img.rotate90(),
            Self::Rotate180 => *img = img.rotate180(),
            Self::Rotate270 => *img = img.rotate270(),
            Self::None => (),
        }
        flip_image(flip_v, flip_h, img);
    }
}

fn flip_image(flip_v: bool, flip_h: bool, img: &mut DynamicImage) {
    if flip_v {
        *img = img.flipv();
    }
    if flip_h {
        *img = img.fliph();
    }
}
