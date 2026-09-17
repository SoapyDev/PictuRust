use crate::picture::Picture;
use image::imageops::FilterType;

use super::parameters::Parameters;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResizeType {
    Exact,
    Thumbnail,
    Fill,
    None,
}

impl ResizeType {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "exact" => Self::Exact,
            "thumbnail" => Self::Thumbnail,
            "fill" => Self::Fill,
            _ => Self::None,
        }
    }

    #[must_use]
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
            Self::Exact => {
                let (resize_width, resize_height) =
                    calculate_dimensions_ratio(params, picture.dimensions);
                picture.image =
                    picture
                        .image
                        .resize_exact(resize_width, resize_height, params.filter);
            }
            Self::Thumbnail => {
                let (resize_width, resize_height) =
                    calculate_dimensions(params, picture.dimensions);
                picture.image = picture.image.thumbnail(resize_width, resize_height);
            }
            Self::Fill => {
                let (resize_width, resize_height) =
                    calculate_dimensions(params, picture.dimensions);
                picture.image =
                    picture
                        .image
                        .resize_to_fill(resize_width, resize_height, params.filter);
            }
            Self::None => {}
        }
    }
}
const fn calculate_dimensions_ratio(params: &Parameters, dimensions: (u32, u32)) -> (u32, u32) {
    match (params.width, params.height, dimensions.0, dimensions.1) {
        (_, _, 0, 0) => (0, 0),
        (None, None, _, _) => dimensions,
        (None, Some(h), _, _) => (dimensions.0 * h / dimensions.1, h),
        (Some(w), None, _, _) => (w, dimensions.1 * w / dimensions.0),
        (Some(w), Some(h), _, _) => (w, h),
    }
}

const fn calculate_dimensions(params: &Parameters, dimensions: (u32, u32)) -> (u32, u32) {
    match (params.width, params.height) {
        (None, None) => dimensions,
        (None, Some(h)) => (dimensions.0, h),
        (Some(w), None) => (w, dimensions.1),
        (Some(w), Some(h)) => (w, h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::format::Format;
    use crate::parameters::rotation::Rotation;
    use std::path::PathBuf;

    fn params(width: Option<u32>, height: Option<u32>) -> Parameters {
        Parameters {
            input_dir: PathBuf::new(),
            recursive: false,
            output_dir: PathBuf::new(),
            width,
            height,
            resize_type: ResizeType::Exact,
            filter: FilterType::Lanczos3,
            format: Format::None,
            quality: 75.0,
            speed: 7,
            rotation: Rotation::None,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }

    #[test]
    fn parses_known_resize_types() {
        assert_eq!(ResizeType::new("Exact"), ResizeType::Exact);
        assert_eq!(ResizeType::new("thumbnail"), ResizeType::Thumbnail);
        assert_eq!(ResizeType::new("FILL"), ResizeType::Fill);
        assert_eq!(ResizeType::new("bogus"), ResizeType::None);
    }

    #[test]
    fn parses_known_filters() {
        assert_eq!(ResizeType::new_filter("triangle"), FilterType::Triangle);
        assert_eq!(ResizeType::new_filter("catmullrom"), FilterType::CatmullRom);
        assert_eq!(ResizeType::new_filter("gaussian"), FilterType::Gaussian);
        assert_eq!(ResizeType::new_filter("nearest"), FilterType::Nearest);
        assert_eq!(ResizeType::new_filter("bogus"), FilterType::Lanczos3);
    }

    #[test]
    fn ratio_keeps_aspect_when_only_width_given() {
        let p = params(Some(500), None);
        assert_eq!(calculate_dimensions_ratio(&p, (1000, 2000)), (500, 1000));
    }

    #[test]
    fn ratio_keeps_aspect_when_only_height_given() {
        let p = params(None, Some(500));
        assert_eq!(calculate_dimensions_ratio(&p, (1000, 2000)), (250, 500));
    }

    #[test]
    fn ratio_uses_exact_dimensions_when_both_given() {
        let p = params(Some(300), Some(150));
        assert_eq!(calculate_dimensions_ratio(&p, (1000, 2000)), (300, 150));
    }

    #[test]
    fn ratio_keeps_original_when_neither_given() {
        let p = params(None, None);
        assert_eq!(calculate_dimensions_ratio(&p, (1000, 2000)), (1000, 2000));
    }

    #[test]
    fn dimensions_fill_in_missing_axis_from_original() {
        let p = params(Some(300), None);
        assert_eq!(calculate_dimensions(&p, (1000, 2000)), (300, 2000));
        let p = params(None, Some(150));
        assert_eq!(calculate_dimensions(&p, (1000, 2000)), (1000, 150));
    }
}
