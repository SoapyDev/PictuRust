use crate::parameters::{format::Format, parameters::Parameters, rotation};
use exif::{Exif, In, Tag};
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct Picture {
    pub format: ImageFormat,
    pub output_path: PathBuf,
    pub dimensions: (u32, u32),
    pub image: DynamicImage,
}

impl Picture {
    pub fn new(source: &PathBuf, params: &Parameters) -> Self {
        let mut picture = create_picture(source, params);
        rotation::set_initial_rotation(get_rotation_code(source), &mut picture);
        set_img_dimensions(&mut picture);
        picture
    }
}

fn create_picture(source: &PathBuf, params: &Parameters) -> Picture {
    let reader = create_img_reader(source);
    Picture {
        output_path: create_output_path(source, params),
        format: get_format(&reader),
        image: get_image(reader, &source),
        dimensions: (0, 0),
    }
}

fn create_img_reader(source: &PathBuf) -> Reader<BufReader<File>> {
    Reader::open(source).expect("Failed to open File")
}

fn create_output_path(source: &PathBuf, params: &Parameters) -> PathBuf {
    match params.format {
        Format::None => params.output_dir.join(source.file_name().unwrap()),
        _ => params
            .output_dir
            .join(get_output_name(source, &params.format)),
    }
}

fn get_output_name(source: &PathBuf, format: &Format) -> PathBuf {
    let mut output_name = source.file_stem().unwrap().to_os_string();
    output_name.push(".");
    output_name.push(format.to_string());
    PathBuf::from(output_name)
}

fn get_format(reader: &Reader<BufReader<File>>) -> ImageFormat {
    reader.format().unwrap_or(ImageFormat::Jpeg)
}

fn get_image(reader: Reader<BufReader<File>>, img_path: &PathBuf) -> DynamicImage {
    match reader.decode() {
        Ok(img) => img,
        Err(_) => {
            println!("Could not decode image: {}", img_path.display());
            DynamicImage::new_rgb8(1, 1)
        }
    }
}

fn get_rotation_code(path: &PathBuf) -> Option<u32> {
    let file = File::open(path).expect("Could not open file");
    let mut bufreader = BufReader::new(file);
    let exifreader = exif::Reader::new();
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        return read_exif(exif);
    }
    None
}

fn read_exif(exif: Exif) -> Option<u32> {
    let orientation = exif.get_field(Tag::Orientation, In::PRIMARY);
    match orientation {
        Some(orientation) => {
            let orientation = orientation.value.get_uint(0).unwrap();
            Some(orientation)
        }
        None => None,
    }
}

fn set_img_dimensions(picture: &mut Picture) {
    let (width, height) = picture.image.dimensions();
    picture.dimensions = (width, height);
}
