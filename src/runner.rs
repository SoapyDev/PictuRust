use jwalk::WalkDir;
use rayon::prelude::*;
use std::{fs::create_dir_all, path::PathBuf};
use std::path::Path;

use crate::{
    parameters::{parameters::Parameters},
    picture::Picture,
};

pub struct Runner;

impl Runner {
    pub fn run(&self, parameters: Parameters) {
        let timer = std::time::Instant::now();
        create_or_validate_output_path(&parameters.output_dir);
        transform_images(&parameters);
        println!("Finished in {}ms", timer.elapsed().as_millis());
    }
}

fn create_or_validate_output_path(path: &PathBuf) {
    if !path.exists() || !path.is_dir() {
        create_dir_all(path).expect("Could not create directory");
    }
}

fn transform_images(params: &Parameters) {
    match params.input_dir.is_file() {
        true => transform_image(&params.input_dir, params),
        false => match params.recursive {
            true => recursive_transform(params),
            false => non_recursive_transform(params),
        },
    }
}

fn recursive_transform(params: &Parameters) {
    WalkDir::new(&params.input_dir)
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

fn non_recursive_transform(params: &Parameters) {
    params
        .input_dir
        .read_dir()
        .expect("Could not read directory")
        .par_bridge()
        .into_par_iter()
        .for_each(|entry| {
            if let Ok(entry) = entry {
                if validate_path_is_image(&entry.path()) {
                    transform_image(&entry.path(), params);
                }
            }
        });
}

fn validate_path_is_image(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => matches!(ext.to_str(), Some("jpg") | Some("jpeg") | Some("png") | Some("tiff") |
            Some("webp") | Some("avif")),
        None => false,
    }
}

fn transform_image(path: &PathBuf, params: &Parameters) {
    let img = Picture::new(path, params);
    match img {
        Ok(mut img) => {
            params.resize_type.alter_img(params, &mut img);
            params.rotation.alter_img(params, &mut img.image);
            params.format.save_img(&mut img);
        }
        Err(e) => {
            eprintln!("Could not open image: {:?} Error : {}", path, e);
        }
    }
}
