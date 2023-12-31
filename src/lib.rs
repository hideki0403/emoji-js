#![deny(clippy::all)]

mod emoji;
use emoji::generate as emojirs;
use napi::{bindgen_prelude::Buffer, Error};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct EmojiOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub flexible_width: Option<bool>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    #[napi(ts_type = "'left' | 'center' | 'right'")]
    pub text_align: Option<String>,
    pub text_size_fixed: Option<bool>,
    pub disable_stretch: Option<bool>,
    pub disable_outline: Option<bool>,
    pub outline_width: Option<u32>,
    pub outline_color: Option<String>,
    pub typeface_file: Option<String>,
    pub typeface_name: Option<String>,
    #[napi(ts_type = "'png' | 'jpeg'")]
    pub format: Option<String>,
    pub quality: Option<u32>,
}

#[napi]
pub fn generate(text: String, options: Option<EmojiOptions>) -> Result<Buffer, Error> {
    let mut emoji = emojirs::new();
    emoji.set_texts(text);

    if let Some(options) = options {
        if let Some(width) = options.width {
            emoji.set_width(width);
        }
        
        if let Some(height) = options.height {
            emoji.set_height(height);
        }
        
        if let Some(flexible_width) = options.flexible_width {
            emoji.set_flexible_width(flexible_width);
        }

        if let Some(color) = options.color {
            let result = emoji.set_color(color);
            if result.is_err() {
                return Err(Error::from_reason(result.unwrap_err()));
            }
        }

        if let Some(background_color) = options.background_color {
            let result = emoji.set_background_color(background_color);
            if result.is_err() {
                return Err(Error::from_reason(result.unwrap_err()));
            }
        }

        if let Some(text_align) = options.text_align {
            let result = emoji.set_text_align_by_string(text_align);
            if result.is_err() {
                return Err(Error::from_reason(result.unwrap_err()));
            }
        }

        if let Some(text_size_fixed) = options.text_size_fixed {
            emoji.set_text_size_fixed(text_size_fixed);
        }

        if let Some(disable_stretch) = options.disable_stretch {
            emoji.set_disable_stretch(disable_stretch);
        }

        if let Some(disable_outline) = options.disable_outline {
            emoji.set_disable_outline(disable_outline);
        }

        if let Some(outline_width) = options.outline_width {
            emoji.set_outline_width(outline_width);
        }

        if let Some(outline_color) = options.outline_color {
            let result = emoji.set_outline_color(outline_color);
            if result.is_err() {
                return Err(Error::from_reason(result.unwrap_err()));
            }
        }

        if let Some(typeface_file) = options.typeface_file {
            emoji.set_typeface_file(typeface_file);
        }

        if let Some(typeface_name) = options.typeface_name {
            emoji.set_typeface_name(typeface_name);
        }

        if let Some(format) = options.format {
            let result = emoji.set_format_by_string(format);
            if result.is_err() {
                return Err(Error::from_reason(result.unwrap_err()));
            }
        }
        
        if let Some(quality) = options.quality {
            emoji.set_quality(quality);
        }
    }
    
    let result = emoji.generate();
    if result.is_err() {
        return Err(Error::from_reason(result.unwrap_err()));
    }
    
    return Ok(Buffer::from(result.unwrap().as_bytes()));
}