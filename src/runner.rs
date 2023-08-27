use jwalk::WalkDir;
use rayon::prelude::*;
use std::{fs::create_dir_all, path::PathBuf};

use crate::{parameters::parameters::Parameters, picture::Picture};

pub struct Runner;

impl Runner {
    pub fn run(&self, parameters: Parameters) {
        let timer = std::time::Instant::now();
        create_or_validate_path(&parameters.output_dir);
        transform_images(&parameters.input_dir, parameters.recursive, &parameters);
        println!("Done!");
        println!("Finished in {}ms", timer.elapsed().as_millis());
    }
}

fn validate_path_is_image(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("jpg") | Some("jpeg") | Some("png") | Some("tiff") => true,
            Some("webp") | Some("avif") => {
                println!("Cannot read {} yet.", ext.to_str().unwrap());
                false
            }
            _ => false,
        },
        None => false,
    }
}

fn create_or_validate_path(path: &PathBuf) {
    if !path.exists() || !path.is_dir() {
        create_dir_all(path).expect("Could not create directory");
    }
}

fn transform_images(path: &PathBuf, is_recursive: bool, params: &Parameters) {
    if path.is_file() {
        transform_image(path, params);
        return;
    }
    if is_recursive {
        recursive_transform(path, params);
    } else {
        non_recursive_transform(path, params);
    }
}

fn recursive_transform(path: &PathBuf, params: &Parameters) {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if validate_path_is_image(&entry.path()) {
                    transform_image(&entry.path(), params);
                }
            }
        });
}

fn non_recursive_transform(path: &PathBuf, params: &Parameters) {
    path.read_dir()
        .expect("Could not read directory")
        .par_bridge()
        .into_par_iter()
        .filter(|entry| entry.is_ok())
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if validate_path_is_image(&entry.path()) {
                    transform_image(&entry.path(), params);
                }
            }
        });
}

fn transform_image(path: &PathBuf, params: &Parameters) {
    let mut img = Picture::new(path, &params.output_dir, &params.format);
    params
        .resize_type
        .resize_image(params.width, params.height, &params.filter, &mut img);
    params
        .rotation
        .rotate_image(params.flip_horizontal, params.flip_vertical, &mut img.image);
    _ = params.format.reformat_image(&mut img, params.quality);
}
