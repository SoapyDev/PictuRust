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
        (None, Some(h), _, _) => ((dimensions.0 * h / dimensions.1), h),
        (Some(w), None, _, _) => (w, (dimensions.1 * w / dimensions.0)),
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

#[cfg(test)]
mod test {
    use image::imageops::FilterType;

    use crate::parameters::resizetype::ResizeType;

    #[test]
    fn new_resize_exact() {
        let resize_type = ResizeType::new("Exact");
        assert_eq!(resize_type, ResizeType::Exact);
    }
    #[test]
    fn new_resize_thumbnail() {
        let resize_type = ResizeType::new("Thumbnail");
        assert_eq!(resize_type, ResizeType::Thumbnail);
    }
    #[test]
    fn new_resize_fill() {
        let resize_type = ResizeType::new("Fill");
        assert_eq!(resize_type, ResizeType::Fill);
    }
    #[test]
    fn new_resize_none() {
        let resize_type = ResizeType::new("None");
        assert_eq!(resize_type, ResizeType::None);
    }
    #[test]
    fn new_filter_triangle() {
        let filter = ResizeType::new_filter("Triangle");
        assert_eq!(filter, FilterType::Triangle);
    }
    #[test]
    fn new_filter_catmullrom() {
        let filter = ResizeType::new_filter("CatmullRom");
        assert_eq!(filter, FilterType::CatmullRom);
    }
    #[test]
    fn new_filter_gaussian() {
        let filter = ResizeType::new_filter("Gaussian");
        assert_eq!(filter, FilterType::Gaussian);
    }
    #[test]
    fn new_filter_nearest() {
        let filter = ResizeType::new_filter("Nearest");
        assert_eq!(filter, FilterType::Nearest);
    }
    #[test]
    fn new_filter_lanczos3() {
        let filter = ResizeType::new_filter("Lanczos3");
        assert_eq!(filter, FilterType::Lanczos3);
    }
    #[test]
    fn new_filter_none() {
        let filter = ResizeType::new_filter("None");
        assert_eq!(filter, FilterType::Lanczos3);
    }
    #[test]
    fn calculate_dimensions() {
        let mut params = super::Parameters::new();
        params.width = Some(100);
        params.height = Some(100);
        let dimensions = super::calculate_dimensions(&params, (100, 100));
        assert_eq!(dimensions, (100, 100));
    }
    #[test]
    fn calculate_dimensions_ratio() {
        let mut params = super::Parameters::new();
        params.width = Some(100);
        let dimensions = super::calculate_dimensions_ratio(&params, (100, 100));
        assert_eq!(dimensions, (100, 100));
    }
    #[test]
    fn calculate_dimensions_ratio_no_params() {
        let params = super::Parameters::new();
        let dimensions = super::calculate_dimensions_ratio(&params, (100, 100));
        assert_eq!(dimensions, (100, 100));
    }
    #[test]
    fn calculate_dimensions_ratio_no_dimensions() {
        let params = super::Parameters::new();
        let dimensions = super::calculate_dimensions_ratio(&params, (0, 0));
        assert_eq!(dimensions, (0, 0));
    }
}
