use crate::parameters::{format::Format, resizetype::ResizeType};

use super::parameters::Parameters;

const TEXT: [&str; 22] = [
    "############################# About ################################\n",
    "Made by SoapyDev",
    "Version : 1.0.0",
    "License : MIT",
    "Description : This is a simple image manipulator made in Rust. It can resize, rotate, flip and convert images in bulk or one at a time.",
    "This program uses the clap, pic, anyhow, image, rayon, jwalk, ravif, rgb, webp and kamadak-exif crates.",
    "\n############################ Commands ##############################\n",
    "--input_dir <i> : The directory where the images are located.",
    "--output_dir <o> : The directory where the images will be saved.",
    "--recursive <R> : If the program should go through the subdirectories of the input directory.",
    "--width <w> : The desired width of the image. If no specification, its calculated.",
    "--height <H> : The desired height of the image. If no specification, its calculated.",
    "--resize_type <t> : The type of resizing to be done. The options are Exact, Thumbnail and Fill. Default is Exact.",
    "--filter <f> : The filter to be used when resizing. The options are Triangle, CatmullRom, Gaussian, Nearest and Lanczos3. Default is Lanczos3.",
    "--format <F> : The format to be used when saving the image. The options are Jpeg, Png, Tiff, Webp, Avif and None. Default is None.",
    "--quality <Q> : The quality of the image when converting to Webp. The options are between 1.0 and 100.00. Default is 70.0.",
    "--speed <S> : The speed to be used when converting to Avif. The options are between 1 and 10. 1 is the slowest but smallest. Default is 7",
    "--rotation <r> : The rotation to be done on the image. The options are 90, 180, 270 and None.",
    "--flip_horizontal <s> : If the image should be flipped horizontally.",
    "--flip_vertical <v> : If the image should be flipped vertically.",
    "--print <p> : The image will be displayed in terminal after the transformation. Options are Sixel, Kitty, Iterm and Blocks.",
    "\n######################### Your Commands ############################\n",
];

pub fn display_user_text(param: &Parameters) {
    display_text(TEXT);
    display_options(param);
}

fn display_text(lines: [&str; 22]) {
    for text in lines.iter() {
        println!("{}", text);
    }
}

fn display_options(param: &Parameters) {
    let mut options: Vec<String> = Vec::with_capacity(15);
    options.push(format!("Input directory : {:?}", param.input_dir));
    options.push(format!("Output directory : {:?}", param.output_dir));
    options.push(format!("Recursive : {:?}", param.recursive));
    options.push(get_value(param.width, "Width", "Calculated"));
    options.push(get_value(param.height, "Height", "Calculated"));
    options.push(format!("Resize type : {:?}", param.resize_type));
    get_filter(param, &mut options);
    options.push(format!("Format : {:?}", param.format));
    get_conversion_param(param, &mut options);
    options.push(format!("Rotation : {:?}", param.rotation));
    options.push(format!("Flip horizontally : {:?}", param.flip_horizontal));
    options.push(format!("Flip vertically : {:?}", param.flip_vertical));
    get_print_value(param, &mut options);
    options.push(format!(
        "\n######################### Your Results #############################\n"
    ));
    for text in options.iter() {
        println!("{}", text);
    }
}

fn get_value(value: Option<u32>, parameter: &str, default_message: &str) -> String {
    if let Some(v) = value {
        format!("{} : {}", parameter, v)
    } else {
        format!("{} : {}", parameter, default_message)
    }
}

fn get_filter(params: &Parameters, options: &mut Vec<String>) {
    match params.resize_type {
        ResizeType::Exact => options.push(format!("{:?}", params.filter)),
        ResizeType::Fill => options.push(format!("{:?}", params.filter)),
        _ => {}
    };
}

fn get_conversion_param(params: &Parameters, options: &mut Vec<String>) {
    match params.format {
        Format::WEBP => options.push(format!("Quality : {:?}", params.quality)),
        Format::AVIF => {
            options.push(format!("Quality : {:?}", params.quality));
            options.push(format!("Speed : {:?}", params.speed));
        }
        _ => {}
    };
}

fn get_print_value(params: &Parameters, options: &mut Vec<String>) {
    match params.print {
        Some(p) => options.push(format!("Print : {:?}", p.to_string())),
        None => options.push("Print : None".to_string()),
    };
}
