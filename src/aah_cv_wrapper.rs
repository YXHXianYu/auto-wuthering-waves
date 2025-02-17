
use std::path::PathBuf;

use aah_cv::template_matching::{find_matches, match_template, MatchTemplateMethod};
use image::DynamicImage;

pub fn template_match(image: &DynamicImage, template: &DynamicImage) -> Option<(i32, i32, f32)> {
    let res = match_template(
        &image.to_luma32f(),
        &template.to_luma32f(),
        MatchTemplateMethod::SumOfSquaredDifferenceNormed,
        false
    );

    let matches = find_matches(
        &res,
        template.width(),
        template.height(),
        MatchTemplateMethod::SumOfSquaredDifferenceNormed,
        0.2
    );

    save_float_buffer_as_png(&res, "res.png").unwrap();
    println!("Matches: {:?}", matches);

    if matches.is_empty() {
        return None;
    }

    let first = matches.first().unwrap();

    let ans = first.location;
    let ans = ((ans.0 + template.width() / 2) as i32, (ans.1 + template.height() / 2) as i32, first.value);

    Some(ans)
}

pub fn open_image(path: &str) -> Result<DynamicImage, anyhow::Error> {
    match image::open(get_assets_dir().join(path)) {
        Ok(image) => Ok(image),
        Err(e) => Err(anyhow::anyhow!("Failed to open image: {}", e))
    }
}

fn get_assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}

use image::{GrayImage, Luma};
use image::ImageBuffer;

#[allow(dead_code)]
fn save_float_buffer_as_png(
    buffer: &ImageBuffer<Luma<f32>, Vec<f32>>,
    output_path: &str
) -> anyhow::Result<()> {
    // 1. 转换 f32 到 u8（假设原始数据范围已经是 0.0-1.0）
    let u8_buffer: GrayImage = ImageBuffer::from_fn(buffer.width(), buffer.height(), |x, y| {
        let pixel = buffer.get_pixel(x, y);
        let value = (pixel[0].clamp(0.0, 1.0) * 255.0) as u8; // 限制范围后转换
        Luma([value])
    });

    // 2. 保存为 PNG
    u8_buffer.save(output_path)?;
    Ok(())
}