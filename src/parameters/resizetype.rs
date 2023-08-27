use image::imageops::FilterType;

use crate::picture::Picture;

#[derive(Debug, PartialEq, Clone)]
pub enum ResizeType {
    Exact,
    Thumbnail,
    Fill,
    Crop,
    None,
}

impl ResizeType {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "exact" => Self::Exact,
            "thumbnail" => Self::Thumbnail,
            "fill" => Self::Fill,
            // "crop" => Self::Crop,
            _ => Self::None,
        }
    }

    pub fn resize_image(
        &self,
        width: Option<u32>,
        height: Option<u32>,
        filter: &FilterType,
        picture: &mut Picture,
    ) {
        match self {
            ResizeType::Exact => {
                let (resize_width, resize_height) =
                    calculate_dimensions_ratio(width, height, picture.dimensions);
                picture.image = picture
                    .image
                    .resize_exact(resize_width, resize_height, *filter);
            }
            ResizeType::Thumbnail => {
                let (resize_width, resize_height) =
                    calculate_dimensions(width, height, picture.dimensions);
                picture.image = picture.image.thumbnail(resize_width, resize_height);
            }
            ResizeType::Fill => {
                let (resize_width, resize_height) =
                    calculate_dimensions(width, height, picture.dimensions);
                picture.image = picture
                    .image
                    .resize_to_fill(resize_width, resize_height, *filter);
            }
            ResizeType::Crop => {}
            ResizeType::None => {}
        }
    }
}
fn calculate_dimensions_ratio(
    width: Option<u32>,
    height: Option<u32>,
    dimensions: (u32, u32),
) -> (u32, u32) {
    match (width, height) {
        (None, None) => dimensions,
        (None, Some(h)) => ((dimensions.0 * h / dimensions.1), h),
        (Some(w), None) => (w, (dimensions.1 * w / dimensions.0)),
        (Some(w), Some(h)) => (w, h),
    }
}

fn calculate_dimensions(
    width: Option<u32>,
    height: Option<u32>,
    dimensions: (u32, u32),
) -> (u32, u32) {
    match (width, height) {
        (None, None) => dimensions,
        (None, Some(h)) => (dimensions.0, h),
        (Some(w), None) => (w, dimensions.1),
        (Some(w), Some(h)) => (w, h),
    }
}
