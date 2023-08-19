use super::utils;
use super::line::Line;
use std::fs::File;
use std::io::Read;
use skia_safe::ISize;
use skia_safe::utils::text_utils::Align as SkTextAlign;
use skia_safe::Color as SkColor;
use skia_safe::EncodedImageFormat as SkEncodedImageFormat;
use skia_safe::Data as SkData;
use skia_safe::surfaces as SkSurfaces;
use skia_safe::typeface::Typeface as SkTypeface;
use skia_safe::FontStyle as SkFontStyle;

pub struct Generator {
    texts: Vec<String>,
    width: f32,
    height: f32,
    color: SkColor,
    background_color: SkColor,
    text_align: SkTextAlign,
    text_size_fixed: bool,
    disable_stretch: bool,
    typeface: SkTypeface,
    format: SkEncodedImageFormat,
    quality: u32,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            texts: Vec::new(),
            width: 128.0,
            height: 128.0,
            color: SkColor::BLACK,
            background_color: SkColor::TRANSPARENT,
            text_align: SkTextAlign::Center,
            text_size_fixed: false,
            disable_stretch: false,
            typeface: SkTypeface::default(),
            format: SkEncodedImageFormat::PNG,
            quality: 100,
        }
    }

    pub fn set_texts(&mut self, texts: String) {
        self.texts = texts.split("\n").map(|text| text.to_string()).collect();
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width as f32;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height as f32;
    }

    pub fn set_color(&mut self, color: String) -> Result<(), String> {
        let result = utils::parse_color_code(color);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.color = result.unwrap();
        return Ok(());
    }

    pub fn set_background_color(&mut self, background_color: String) -> Result<(), String> {
        let result = utils::parse_color_code(background_color);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.background_color = result.unwrap();
        return Ok(());
    }

    pub fn set_text_align(&mut self, text_align: SkTextAlign) {
        self.text_align = text_align;
    }

    pub fn set_text_align_by_string(&mut self, text_align: String) -> Result<(), String> {
        let result = utils::parse_text_align(text_align);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.text_align = result.unwrap();
        return Ok(());
    }

    pub fn set_text_size_fixed(&mut self, text_size_fixed: bool) {
        self.text_size_fixed = text_size_fixed;
    }

    pub fn set_disable_stretch(&mut self, disable_stretch: bool) {
        self.disable_stretch = disable_stretch;
    }

    pub fn set_typeface_file(&mut self, path: String) {
        let file = File::open(path);
        if file.is_err() {
            return;
        }

        let mut buf = Vec::new();
        let result = file.unwrap().read_to_end(&mut buf);
        if result.is_err() {
            return;
        }
        
        let tf = SkTypeface::from_data(SkData::new_copy(&buf), 0);
        if tf.is_none() {
            return;
        }

        self.typeface = tf.unwrap();
    }

    pub fn set_typeface_name(&mut self, name: String) {
        let tf = SkTypeface::from_name(name, SkFontStyle::normal());
        if tf.is_none() {
            return;
        }

        self.typeface = tf.unwrap();
    }

    pub fn set_format(&mut self, format: SkEncodedImageFormat) {
        self.format = format;
    }

    pub fn set_format_by_string(&mut self, format: String) -> Result<(), String> {
        let result = utils::parse_image_format(format);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        self.format = result.unwrap();
        return Ok(());
    }

    pub fn set_quality(&mut self, quality: u32) {
        self.quality = quality;
    }

    pub fn generate(&self) -> SkData {
        let mut surface = SkSurfaces::raster_n32_premul(ISize::new(self.width as i32, self.height as i32)).unwrap();
        let mut canvas = surface.canvas();
        canvas.clear(self.background_color);

        let line_height = self.height / self.texts.len() as f32;

        // 行ボックスを作成
        let mut lines = Vec::new();
        for text in &self.texts {
            let mut line = Line::new();
            line.set_width(self.width);
            line.set_line_height(line_height);
            line.set_text(text.to_string());
            line.set_typeface(self.typeface.clone());
            line.set_text_align(self.text_align);
            line.set_color(self.color);
            line.set_disable_stretch(self.disable_stretch);

            // 高さ・幅を計測
            line.measure(None);

            lines.push(line);
        }

        // サイズ固定モード: 最小テキストサイズで再計算
        if self.text_size_fixed && lines.len() > 1 {
            let min_text_size = lines.iter().map(|line| line.get_text_size()).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            for line in &mut lines {
                line.measure(Some(min_text_size));
            }
        }

        // テキストを描画
        for (i, line) in lines.iter_mut().enumerate() {
            line.draw(&mut canvas, line_height * i as f32);
        }

        // エンコード
        let image = surface.image_snapshot();
        return image.encode(None, self.format, self.quality).unwrap();
    }
}