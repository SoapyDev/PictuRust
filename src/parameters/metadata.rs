use crate::picture::Picture;

use super::parameters::Parameters;

pub struct Metadata {}

impl Metadata {
    pub fn copy_metadata(params: &Parameters, picture: &Picture) {
        if !params.keep_metadata {
            return;
        }
        let metadata = rexiv2::Metadata::new_from_path(&picture.source_path);
        match metadata {
            Ok(metadata) => {
                _ = metadata.save_to_file(&picture.output_path);
            }
            Err(_) => {}
        }
    }
}
