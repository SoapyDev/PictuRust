use pic::{options::Options, support::Protocol};

use crate::picture::Picture;

use super::parameters::Parameters;

pub fn new(s: &str) -> Protocol {
    match s.to_lowercase().trim().as_ref() {
        "sixel" => Protocol::Sixel,
        "kitty" => Protocol::Kitty,
        "iterm" => Protocol::Iterm,
        _ => Protocol::Blocks,
    }
}

pub fn img_preview(img: &Picture, params: &Parameters) {
    if params.print.is_none() {
        return;
    }
    let mut options = Options::new(vec![img.output_path.clone()]);
    options.protocol = params.print;
    if let Err(err) = pic::previewer::preview(&mut std::io::stdout(), &mut options) {
        eprintln!("Error: {}", err);
    };
}
