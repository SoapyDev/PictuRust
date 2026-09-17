use jwalk::WalkDir;
use rayon::prelude::*;
use std::path::Path;
use std::{fs::create_dir_all, path::PathBuf};

use crate::{parameters::parameters::Parameters, picture::Picture};

pub struct Runner;

impl Runner {
    /// # Panics
    /// Panics if the global rayon thread pool has already been initialized
    /// elsewhere in the process.
    pub fn run(&self, parameters: &Parameters) {
        rayon::ThreadPoolBuilder::new()
            .num_threads(parameters.threads)
            .build_global()
            .expect("Could not set thread pool size");
        let timer = std::time::Instant::now();
        create_or_validate_output_path(&parameters.output_dir);
        transform_images(parameters);
        let elapsed = timer.elapsed().as_millis();
        println!("Finished in {elapsed}ms");
    }
}

fn create_or_validate_output_path(path: &PathBuf) {
    if !path.exists() || !path.is_dir() {
        create_dir_all(path).expect("Could not create directory");
    }
}

fn transform_images(params: &Parameters) {
    if params.input_dir.is_file() {
        transform_image(&params.input_dir, params);
    } else if params.recursive {
        recursive_transform(params);
    } else {
        non_recursive_transform(params);
    }
}

fn recursive_transform(params: &Parameters) {
    WalkDir::new(&params.input_dir)
        .into_iter()
        .par_bridge()
        .for_each(|entry| {
            if let Ok(entry) = entry
                && validate_path_is_image(&entry.path())
            {
                transform_image(&entry.path(), params);
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
            if let Ok(entry) = entry
                && validate_path_is_image(&entry.path())
            {
                transform_image(&entry.path(), params);
            }
        });
}

fn validate_path_is_image(path: &Path) -> bool {
    path.extension().is_some_and(|ext| {
        matches!(
            ext.to_str(),
            Some("jpg" | "jpeg" | "png" | "tiff" | "webp" | "avif")
        )
    })
}

/// Runs the full decode -> resize -> rotate/flip -> encode pipeline for a single
/// image. Exposed for benchmarks and tests that need to exercise the pipeline
/// without going through the CLI.
pub fn transform_single(path: &PathBuf, params: &Parameters) {
    transform_image(path, params);
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
            let path = path.display();
            eprintln!("Could not open image: {path} Error : {e}");
        }
    }
}
