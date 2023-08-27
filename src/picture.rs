use std::{fs::File, io::BufReader, path::PathBuf};

use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};

use crate::parameters::format::Format;

pub struct Picture {
    pub format: ImageFormat,
    pub output_path: PathBuf,
    pub dimensions: (u32, u32),
    pub image: DynamicImage,
}

impl Picture {
    pub fn new(source: &PathBuf, output_path: &PathBuf, format: &Format) -> Self {
        let reader = Reader::open(source).expect("Failed to open image");
        let mut img = Self {
            output_path: create_output_path(&source, &output_path, format),
            format: get_format(&reader),
            image: get_image(reader),
            dimensions: (0, 0),
        };
        set_initial_rotation(get_rotation_code(source), &mut img);
        img.dimensions = img.image.dimensions();
        img
    }
}

fn create_output_path(source: &PathBuf, output: &PathBuf, format: &Format) -> PathBuf {
    match format {
        Format::None => output.join(source.file_name().unwrap()),
        _ => output.join(get_output_name(source, format)),
    }
}

fn get_format(reader: &Reader<BufReader<File>>) -> ImageFormat {
    reader.format().unwrap_or(ImageFormat::Jpeg)
}

fn get_image(reader: Reader<BufReader<File>>) -> DynamicImage {
    if let Ok(img) = reader.decode() {
        return img;
    } else {
        return DynamicImage::new_rgb8(1, 1);
    }
}
fn get_output_name(source: &PathBuf, format: &Format) -> PathBuf {
    let mut output_name = source.file_stem().unwrap().to_os_string();
    output_name.push(".");
    output_name.push(format.to_string());
    PathBuf::from(output_name)
}
fn get_rotation_code(path: &PathBuf) -> Option<u32> {
    let file = std::fs::File::open(path).expect("Could not open file");
    let mut bufreader = BufReader::new(file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader);

    match exif {
        Ok(exif) => {
            let orientation = exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY);
            match orientation {
                Some(orientation) => {
                    let code = orientation.value.get_uint(0).unwrap();
                    Some(code)
                }
                None => None,
            }
        }
        Err(_) => None,
    }
}

fn set_initial_rotation(code: Option<u32>, picture: &mut Picture) {
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
