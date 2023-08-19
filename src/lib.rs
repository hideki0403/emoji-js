#![deny(clippy::all)]

mod emoji;
use emoji::generate as emojirs;
use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct EmojiOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    #[napi(ts_type = "'left' | 'center' | 'right'")]
    pub text_align: Option<String>,
    pub text_size_fixed: Option<bool>,
    pub disable_stretch: Option<bool>,
    pub typeface_file: Option<String>,
    pub typeface_name: Option<String>,
    #[napi(ts_type = "'png' | 'jpeg' | 'webp' | 'gif'")]
    pub format: Option<String>,
    pub quality: Option<u32>,
}

#[napi]
pub fn generate(text: String, options: Option<EmojiOptions>) -> Buffer {
    let mut emoji = emojirs::new();
    emoji.set_texts(text);

    if let Some(options) = options {
        if let Some(width) = options.width {
            emoji.set_width(width);
        }
        if let Some(height) = options.height {
            emoji.set_height(height);
        }
        if let Some(color) = options.color {
            emoji.set_color(color);
        }
        if let Some(background_color) = options.background_color {
            emoji.set_background_color(background_color);
        }
        if let Some(text_align) = options.text_align {
            emoji.set_text_align_by_string(text_align);
        }
        if let Some(text_size_fixed) = options.text_size_fixed {
            emoji.set_text_size_fixed(text_size_fixed);
        }
        if let Some(disable_stretch) = options.disable_stretch {
            emoji.set_disable_stretch(disable_stretch);
        }
        if let Some(typeface_file) = options.typeface_file {
            emoji.set_typeface_file(typeface_file);
        }
        if let Some(typeface_name) = options.typeface_name {
            emoji.set_typeface_name(typeface_name);
        }
        if let Some(format) = options.format {
            emoji.set_format_by_string(format);
        }
        if let Some(quality) = options.quality {
            emoji.set_quality(quality);
        }
    }
    
    return Buffer::from(emoji.generate().as_bytes())
}