use super::parameters::Parameters;
use crate::picture::Picture;
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

#[cfg(test)]
mod test {
    use image::DynamicImage;

    #[test]
    fn new_no_rotation() {
        let rotation = super::Rotation::new("None");
        assert_eq!(rotation, super::Rotation::None);
    }
    #[test]
    fn new_rotate_90() {
        let rotation = super::Rotation::new("90");
        assert_eq!(rotation, super::Rotation::Rotate90);
    }
    #[test]
    fn new_rotate_180() {
        let rotation = super::Rotation::new("180");
        assert_eq!(rotation, super::Rotation::Rotate180);
    }
    #[test]
    fn new_rotate_270() {
        let rotation = super::Rotation::new("270");
        assert_eq!(rotation, super::Rotation::Rotate270);
    }
    #[test]
    fn rotate_90() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let rotation = super::Rotation::Rotate90;
        super::rotate(&mut img, rotation);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1).rotate90());
    }
    #[test]
    fn rotate_180() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let rotation = super::Rotation::Rotate180;
        super::rotate(&mut img, rotation);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1).rotate180());
    }
    #[test]
    fn rotate_270() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let rotation = super::Rotation::Rotate270;
        super::rotate(&mut img, rotation);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1).rotate270());
    }
    #[test]
    fn rotate_none() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let rotation = super::Rotation::None;
        super::rotate(&mut img, rotation);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1));
    }
    #[test]
    fn flip_vertically() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let flip_v = true;
        super::flip_vertically(flip_v, &mut img);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1).flipv());
    }
    #[test]
    fn flip_horizontally() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let flip_h = true;
        super::flip_horizontally(flip_h, &mut img);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1).fliph());
    }
    #[test]
    fn flip_none() {
        let mut img = DynamicImage::new_rgb8(1, 1);
        let flip_h = false;
        let flip_v = false;
        super::flip_horizontally(flip_h, &mut img);
        super::flip_vertically(flip_v, &mut img);
        assert_eq!(img, DynamicImage::new_rgb8(1, 1));
    }
}
