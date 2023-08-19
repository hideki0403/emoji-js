use skia_safe::Color as SkColor;
use skia_safe::utils::text_utils::Align as SkTextAlign;
use skia_safe::EncodedImageFormat as SkEncodedImageFormat;

pub fn parse_color_code(f_color: String) -> Result<SkColor, String> {
    let mut color = f_color.to_string();
    
    if color.starts_with("#") {
        color = color[1..].to_string();
    }

    // 色コードの文字列が16進数でなければエラー
    if !color.chars().all(|c| c.is_digit(16)) {
        return Err(format!("Invalid color code: {}", f_color));
    }

    // 色コードの桁数が6桁でも8桁でもなければエラー
    if color.len() != 6 && color.len() != 8 {
        return Err(format!("color code must be 6 or 8 digits (eg. #000000, #000000FF)"));
    }

    // もし色コードが6桁なら8桁にする
    if color.len() == 6 {
        color.push_str("FF");
    }

    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    let a = u8::from_str_radix(&color[6..8], 16).unwrap();

    return Ok(SkColor::from_argb(a, r, g, b));
}

pub fn parse_text_align(text_align: String) -> Result<SkTextAlign, String> {
    match text_align.as_str() {
        "left" => Ok(SkTextAlign::Left),
        "center" => Ok(SkTextAlign::Center),
        "right" => Ok(SkTextAlign::Right),
        _ => Err(format!("Invalid text align: {}", text_align)),
    }
}

pub fn parse_image_format(format: String) -> Result<SkEncodedImageFormat, String> {
    match format.as_str() {
        "png" => Ok(SkEncodedImageFormat::PNG),
        "jpeg" => Ok(SkEncodedImageFormat::JPEG),
        "webp" => Ok(SkEncodedImageFormat::WEBP),
        "gif" => Ok(SkEncodedImageFormat::GIF),
        _ => Err(format!("Invalid image format: {}", format)),
    }
}