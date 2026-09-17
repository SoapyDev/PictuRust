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
    author = "SoapyDev",
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
    #[arg(short='W', long, default_value = Option::None, required = false)]
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
    #[arg(short = 'T', long, default_value_t = default_thread_count(), value_parser = threads_in_range, required = false)]
    pub threads: usize,
}

impl Parameters {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self::parse()
    }

    #[must_use]
    pub fn new_with_display() -> Self {
        let param = Self::new();
        display::display_user_text(&param);
        param
    }
}

// These parsers always succeed today, but clap's `value_parser` requires a
// `Result`-returning function signature to report arg-parsing errors.
#[allow(clippy::unnecessary_wraps)]
fn get_filter(s: &str) -> Result<imageops::FilterType, Error> {
    Ok(ResizeType::new_filter(s))
}

#[allow(clippy::unnecessary_wraps)]
fn get_type(s: &str) -> Result<ResizeType, Error> {
    Ok(ResizeType::new(s))
}

#[allow(clippy::unnecessary_wraps)]
fn get_format(s: &str) -> Result<Format, Error> {
    Ok(Format::new(s))
}

#[allow(clippy::unnecessary_wraps)]
fn get_rotation(s: &str) -> Result<Rotation, Error> {
    Ok(Rotation::new(s))
}

const EFFORT_RANGE: RangeInclusive<u8> = 1..=10;
fn get_effort(s: &str) -> Result<u8, Error> {
    let effort = s.parse::<u8>().expect("Effort is not between 1 and 10");
    if EFFORT_RANGE.contains(&effort) {
        Ok(effort)
    } else {
        Err(Error::msg("Effort is not between 1 and 10"))
    }
}

const QUALITY_RANGE: RangeInclusive<f32> = 1.0..=100.0;
fn quality_in_range(s: &str) -> Result<f32, String> {
    let quality = s.parse::<f32>().expect("Not a float");
    if QUALITY_RANGE.contains(&quality) {
        Ok(quality)
    } else {
        Err("Quality is not a between 1.0 and 100.0".to_string())
    }
}

#[must_use]
pub fn default_thread_count() -> usize {
    std::thread::available_parallelism().map_or(1, std::num::NonZero::get)
}

fn threads_in_range(s: &str) -> Result<usize, Error> {
    let threads = s
        .parse::<usize>()
        .map_err(|_| Error::msg("Not a valid number"))?;
    let max = default_thread_count();
    if (1..=max).contains(&threads) {
        Ok(threads)
    } else {
        Err(Error::msg(format!(
            "Threads must be between 1 and {max} (the number of threads available on this machine)"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_thread_count_is_at_least_one() {
        assert!(default_thread_count() >= 1);
    }

    #[test]
    fn threads_in_range_accepts_one_and_the_machine_max() {
        assert_eq!(threads_in_range("1").unwrap(), 1);
        let max = default_thread_count();
        assert_eq!(threads_in_range(&max.to_string()).unwrap(), max);
    }

    #[test]
    fn threads_in_range_rejects_zero_and_above_max() {
        assert!(threads_in_range("0").is_err());
        let too_many = default_thread_count() + 1;
        assert!(threads_in_range(&too_many.to_string()).is_err());
    }

    #[test]
    fn threads_in_range_rejects_non_numeric_input() {
        assert!(threads_in_range("not a number").is_err());
    }
}
