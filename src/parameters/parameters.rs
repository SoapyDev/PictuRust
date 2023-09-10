use anyhow::Error;
use clap::Parser;
use format::Format;
use image::imageops;
use resizetype::ResizeType;
use rotation::Rotation;
use std::{ops::RangeInclusive, path::PathBuf};

use super::{display, format, resizetype, rotation};

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
    #[arg(short = 'S', long, default_value = "7", value_parser=get_effort, required = false)]
    pub speed: u8,
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

    pub fn new_with_display() -> Self {
        let param = Self::new();
        display::display_user_text(&param);
        param
    }
}

fn get_filter(s: &str) -> Result<imageops::FilterType, Error> {
    Ok(ResizeType::new_filter(s))
}

fn get_type(s: &str) -> Result<ResizeType, Error> {
    Ok(ResizeType::new(s))
}

fn get_format(s: &str) -> Result<Format, Error> {
    Ok(Format::new(s))
}

fn get_rotation(s: &str) -> Result<Rotation, Error> {
    Ok(Rotation::new(s))
}

const EFFORT_RANGE: RangeInclusive<u8> = 1..=10;
fn get_effort(s: &str) -> Result<u8, Error> {
    let effort = s.parse::<u8>().expect("Effort is not between 1 and 10");
    match EFFORT_RANGE.contains(&effort) {
        true => Ok(effort),
        false => Err(Error::msg("Effort is not between 1 and 10")),
    }
}

const QUALITY_RANGE: RangeInclusive<f32> = 1.0..=100.0;
fn quality_in_range(s: &str) -> Result<f32, String> {
    let quality = s.parse::<f32>().expect("Not a float");
    match QUALITY_RANGE.contains(&quality) {
        true => Ok(quality),
        false => Err("Quality is not a between 1.0 and 100.0".to_string()),
    }
}