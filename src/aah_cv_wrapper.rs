
use std::path::PathBuf;

use aah_cv::template_matching::{find_matches, match_template, MatchTemplateMethod};
use image::DynamicImage;


pub fn template_match(image: &DynamicImage, template: &DynamicImage) -> (i32, i32) {
    let res = match_template(
        &image.to_luma32f(),
        &template.to_luma32f(),
        MatchTemplateMethod::CorrelationCoefficientNormed,
        false
    );

    let matches = find_matches(
        &res,
        template.width(),
        template.height(),
        MatchTemplateMethod::CorrelationCoefficientNormed,
        0.9
    );

    assert!(!matches.is_empty());

    let ans = matches.first().unwrap().clone().location;

    ((ans.0 + template.width() / 2) as i32, (ans.1 + template.height() / 2) as i32)
}

pub fn open_image(path: &str) -> DynamicImage {
    image::open(get_assets_dir().join(path)).unwrap()
}

fn get_assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}