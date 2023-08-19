use skia_safe::Color as SkColor;
use skia_safe::utils::text_utils::Align as SkTextAlign;
use skia_safe::EncodedImageFormat as SkEncodedImageFormat;

pub fn parse_color_code(color: String) -> SkColor {
    let mut color = color;
    
    if color.starts_with("#") {
        color = color[1..].to_string();
    }

    // 色コードの文字列が16進数でなければエラー
    if !color.chars().all(|c| c.is_digit(16)) {
        panic!("Invalid color code: {}", color);
    }

    // 色コードの桁数が6桁でも8桁でもなければエラー
    if color.len() != 6 && color.len() != 8 {
        panic!("color code must be 6 or 8 digits (eg. #000000, #000000FF)");
    }

    // もし色コードが6桁なら8桁にする
    if color.len() == 6 {
        color.push_str("FF");
    }

    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    let a = u8::from_str_radix(&color[6..8], 16).unwrap();

    return SkColor::from_argb(a, r, g, b);
}

pub fn parse_text_align(text_align: String) -> SkTextAlign {
    match text_align.as_str() {
        "left" => SkTextAlign::Left,
        "center" => SkTextAlign::Center,
        "right" => SkTextAlign::Right,
        _ => panic!("Invalid text align: {}", text_align),
    }
}

pub fn parse_image_format(format: String) -> SkEncodedImageFormat {
    match format.as_str() {
        "png" => SkEncodedImageFormat::PNG,
        "jpeg" => SkEncodedImageFormat::JPEG,
        "webp" => SkEncodedImageFormat::WEBP,
        "gif" => SkEncodedImageFormat::GIF,
        _ => panic!("Invalid image format: {}", format),
    }
}