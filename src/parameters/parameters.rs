use clap::Parser;
use format::Format;
use image::imageops;
use resizetype::ResizeType;
use rotation::Rotation;
use std::{ops::RangeInclusive, path::PathBuf};

use super::{format, resizetype, rotation};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "This is a simple image manipulator made in Rust. It can resize, rotate, flip and convert images in bulk or one at a time."
)]
pub struct Parameters {
    #[arg(short, long, required = true)]
    pub input_dir: PathBuf,
    #[arg(short = 'R', long, default_value = "false", required = false)]
    pub recursive: bool,
    #[arg(short, long, required = true)]
    pub output_dir: PathBuf,
    #[arg(short, long, default_value = Option::None, required = false)]
    pub width: Option<u32>,
    #[arg(short='H', long, default_value = Option::None ,required = false)]
    pub height: Option<u32>,
    #[arg(short='t', long ,default_value = "Exact", value_parser=get_type, required = false)]
    pub resize_type: ResizeType,
    #[arg(short, long ,default_value = "Lanczos3", value_parser=get_filter, required = false)]
    pub filter: imageops::FilterType,
    #[arg(short='F', long, default_value = "None", value_parser=get_format, required = false)]
    pub format: Format,
    #[arg(short='Q', long="quality", default_value ="75.0", value_parser=quality_in_range, required = false)]
    pub quality: f32,
    #[arg(short, long, default_value = "0", value_parser=get_rotation, required = false)]
    pub rotation: Rotation,
    #[arg(short = 's', long, default_value = "false", required = false)]
    pub flip_horizontal: bool,
    #[arg(short = 'v', long, default_value = "false", required = false)]
    pub flip_vertical: bool,
}

impl Parameters {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn display(&self) {
        println!("############################# About ################################\n");

        println!("Made by SoapyDev");
        println!("Version : 1.0.0");
        println!("License : MIT");
        println!(
            "Description : This is a simple image manipulator made in Rust. 
        It can resize, rotate, flip and convert images in bulk or one at a time."
        );
        println!("Implementation : This program uses the clap, anyhow, image, rayon, jwalk and kamadak-exif crates.");

        println!("\n############################ Commands ##############################\n");

        println!("--input_dir <i> : The directory where the images are located.");
        println!("--output_dir <o> : The directory where the images will be saved.");
        println!("--recursive <r> : If the program should go through the subdirectories of the input directory.");
        println!("--width <w> : The desired width of the image.");
        println!("--height <h> : The desired height of the image.");
        println!("--resize_type <t> : The type of resizing to be done. The options are Exact, Thumbnail, Fill and Crop.");
        println!("--filter <f> : The filter to be used when resizing. The options are Triangle, CatmullRom, Gaussian, Nearest and Lanczos3.");
        println!("--format <F> : The format to be used when saving the image. The options are Jpeg, Png, Tiff, Webp and None.");
        println!("--quality <Q> : The quality of the image when converting to Webp. The options are between 1.0 and 100.00.");
        println!("--rotation <r> : The rotation to be done on the image. The options are 90, 180, 270 and None.");
        println!("--flip_horizontal <s> : If the image should be flipped horizontally.");
        println!("--flip_vertical <v> : If the image should be flipped vertically.");

        println!("\n######################### Your Commands ############################\n");

        println!("Input directory : {:?}", self.input_dir);
        println!("Output directory : {:?}", self.output_dir);
        println!("Recursive : {:?}", self.recursive);
        println!("Width : {:?}", self.width);
        println!("Height : {:?}", self.height);
        println!("Resize type : {:?}", self.resize_type);
        println!("Filter : {:?}", self.filter);
        println!("Format : {:?}", self.format);
        println!("Quality : {:?}", self.quality);
        println!("Rotation : {:?}", self.rotation);
        println!("Flip_horizontal : {:?}", self.flip_horizontal);
        println!("Flip_vertical : {:?}", self.flip_vertical);

        println!("\n######################### Your Results #############################\n");
    }
}

fn get_filter(s: &str) -> Result<imageops::FilterType, anyhow::Error> {
    match s.to_lowercase().trim() {
        "triangle" => Ok(imageops::FilterType::Triangle),
        "catmullrom" => Ok(imageops::FilterType::CatmullRom),
        "gaussian" => Ok(imageops::FilterType::Gaussian),
        "nearest" => Ok(imageops::FilterType::Nearest),
        &_ => Ok(imageops::FilterType::Lanczos3),
    }
}
fn get_type(s: &str) -> Result<ResizeType, anyhow::Error> {
    Ok(ResizeType::new(s))
}
fn get_format(s: &str) -> Result<Format, anyhow::Error> {
    Ok(Format::new(s))
}
fn get_rotation(s: &str) -> Result<Rotation, anyhow::Error> {
    Ok(Rotation::new(s))
}

const QUALITY_RANGE: RangeInclusive<f32> = 1.0..=100.0;
fn quality_in_range(s: &str) -> Result<f32, String> {
    let f = s.parse::<f32>().map_err(|_| "not a float")?;
    if QUALITY_RANGE.contains(&f) {
        Ok(f)
    } else {
        Err("not in range".to_string())
    }
}
