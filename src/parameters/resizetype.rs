use crate::picture::Picture;
use image::imageops::FilterType;

use super::parameters::Parameters;

#[derive(Debug, PartialEq, Clone)]
pub enum ResizeType {
    Exact,
    Thumbnail,
    Fill,
    None,
}

impl ResizeType {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "exact" => Self::Exact,
            "thumbnail" => Self::Thumbnail,
            "fill" => Self::Fill,
            _ => Self::None,
        }
    }

    pub fn new_filter(s: &str) -> FilterType {
        match s.to_lowercase().trim() {
            "triangle" => FilterType::Triangle,
            "catmullrom" => FilterType::CatmullRom,
            "gaussian" => FilterType::Gaussian,
            "nearest" => FilterType::Nearest,
            _ => FilterType::Lanczos3,
        }
    }

    pub fn alter_img(&self, params: &Parameters, picture: &mut Picture) {
        match self {
            ResizeType::Exact => {
                let (resize_width, resize_height) =
                    calculate_dimensions_ratio(params, picture.dimensions);
                picture.image =
                    picture
                        .image
                        .resize_exact(resize_width, resize_height, params.filter);
            }
            ResizeType::Thumbnail => {
                let (resize_width, resize_height) =
                    calculate_dimensions(params, picture.dimensions);
                picture.image = picture.image.thumbnail(resize_width, resize_height);
            }
            ResizeType::Fill => {
                let (resize_width, resize_height) =
                    calculate_dimensions(params, picture.dimensions);
                picture.image =
                    picture
                        .image
                        .resize_to_fill(resize_width, resize_height, params.filter);
            }
            ResizeType::None => {}
        }
    }
}
fn calculate_dimensions_ratio(params: &Parameters, dimensions: (u32, u32)) -> (u32, u32) {
    match (params.width, params.height, dimensions.0, dimensions.1) {
        (_, _, 0, 0) => (0, 0),
        (None, None, _, _) => dimensions,
        (None, Some(h), _, _) => (dimensions.0 * h / dimensions.1, h),
        (Some(w), None, _, _) => (w, dimensions.1 * w / dimensions.0),
        (Some(w), Some(h), _, _) => (w, h),
    }
}

fn calculate_dimensions(params: &Parameters, dimensions: (u32, u32)) -> (u32, u32) {
    match (params.width, params.height) {
        (None, None) => dimensions,
        (None, Some(h)) => (dimensions.0, h),
        (Some(w), None) => (w, dimensions.1),
        (Some(w), Some(h)) => (w, h),
    }
}
