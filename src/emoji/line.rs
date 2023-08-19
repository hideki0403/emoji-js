use skia_safe::Point;
use skia_safe::utils::text_utils::Align as SkTextAlign;
use skia_safe::typeface::Typeface as SkTypeface;
use skia_safe::Color as SkColor;
use skia_safe::scalar as SkScalar;
use skia_safe::Rect as SkRect;
use skia_safe::paint::Paint as SkPaint;
use skia_safe::Font as SkFont;
use skia_safe::colors as SkColors;
use skia_safe::TextEncoding as SkTextEncoding;
use skia_safe::Canvas as SkCanvas;
use skia_safe::utils::text_utils as SkTextUtils;

pub struct MeasureSpec {
    text_size: SkScalar,
    text_scale_x: SkScalar,
    bounds: SkRect,
}

impl MeasureSpec {
    pub fn new() -> Self {
        Self {
            text_size: 128.0,
            text_scale_x: 1.0,
            bounds: SkRect::new(0.0, 0.0, 128.0, 128.0),
        }
    }
}

pub struct Line {
    text: String,
    width: SkScalar,
    line_height: SkScalar,
    typeface: SkTypeface,
    text_align: SkTextAlign,
    color: SkColor,
    disable_stretch: bool,
    spec: MeasureSpec,
}

impl Line {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            width: 128.0,
            line_height: 128.0,
            typeface: SkTypeface::default(),
            text_align: SkTextAlign::Center,
            color: SkColor::BLACK,
            disable_stretch: false,
            spec: MeasureSpec::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_width(&mut self, width: SkScalar) {
        self.width = width;
    }

    pub fn set_line_height(&mut self, line_height: SkScalar) {
        self.line_height = line_height;
    }

    pub fn set_typeface(&mut self, typeface: SkTypeface) {
        self.typeface = typeface;
    }

    pub fn set_text_align(&mut self, text_align: SkTextAlign) {
        self.text_align = text_align;
    }

    pub fn set_color(&mut self, color: SkColor) {
        self.color = color;
    }

    pub fn set_disable_stretch(&mut self, disable_stretch: bool) {
        self.disable_stretch = disable_stretch;
    }

    pub fn measure(&mut self, text_size: Option<SkScalar>) {
        if text_size.is_some() {
            self.measure_size_fixed(text_size.unwrap());
        } else {
            self.measure_adjusted();
        }
    }

    pub fn draw(&mut self, canvas: &mut SkCanvas, y: SkScalar) {
        let paint = self.prepare_paint_for_draw();
        let font = self.prepare_font_for_draw();

        // for X-axis
        let x;
        match self.text_align {
            SkTextAlign::Left => {
                x = -self.spec.bounds.left;
            },
            SkTextAlign::Center => {
                if self.spec.text_scale_x < 1.0 {
                    x = -self.spec.bounds.left;
                } else {
                    x = (self.width - self.spec.bounds.width()) / 2.0 - self.spec.bounds.left;
                }
            },
            SkTextAlign::Right => {
                if self.spec.text_scale_x < 1.0 {
                    x = -self.spec.bounds.left;
                } else {
                    x = self.width - self.spec.bounds.width() - self.spec.bounds.left;
                }
            },
        }

        // for Y-axis
        let offset_y = (self.line_height - self.spec.bounds.height()) / 2.0;

        SkTextUtils::draw_str(canvas, &self.text, Point::new(x, y - self.spec.bounds.top + offset_y), &font, &paint, self.text_align);
    }

    // MeasureSpec

    // テキストサイズ可変モード
    pub fn measure_adjusted(&mut self) {
        let paint = self.prepare_paint_for_measure();
        let mut font = self.prepare_font_for_measure();
        let mut bounds;

        let mut min_text_size = self.line_height * 0.9;
        let max_text_size = self.line_height * 10.0;

        let mut prev_bounds = SkRect::new_empty();
        let mut prev_text_size = 0.0;

        if !self.text.is_empty() {
            // 非伸縮モード: 初期フォントサイズを調整
            if self.disable_stretch {
                let mut i = min_text_size;
                loop {
                    if i < 0.0 { break }
                    
                    font.set_size(i);
                    bounds = font.measure_text(self.text.as_bytes(), SkTextEncoding::UTF8, Some(&paint)).1;

                    if bounds.width() < self.width {
                        min_text_size = i;
                        break;
                    }

                    i -= 0.5;
                }
            }

            let mut i = min_text_size;
            loop {
                if i > max_text_size { break }

                font.set_size(i);
                bounds = font.measure_text(self.text.as_bytes(), SkTextEncoding::UTF8, Some(&paint)).1;

                if bounds.height() > self.line_height { break }
                if self.disable_stretch && bounds.width() > self.width { break }

                prev_text_size = i;
                prev_bounds = bounds;

                i += 0.5;
            }
        }

        self.spec.text_scale_x = 1.0;
        self.spec.text_size = prev_text_size;
        self.spec.bounds = prev_bounds;

        // 横方向圧縮が必要な場合: 圧縮率の調整
        if prev_bounds.width() > self.width {
            font.set_size(prev_text_size);

            let mut i = self.width / prev_bounds.width();
            loop {
                if i < 0.0 { break }

                font.set_scale_x(i);
                bounds = font.measure_text(self.text.as_bytes(), SkTextEncoding::UTF8, Some(&paint)).1;

                if bounds.width() <= self.width {
                    self.spec.bounds = bounds;
                    self.spec.text_scale_x = i;
                    break;
                }

                i -= 0.0001;
            }
        }
    }

    // テキストサイズ固定モード
    pub fn measure_size_fixed(&mut self, text_size: SkScalar) {
        let mut font = self.prepare_font_for_measure();
        let mut bounds;

        font.set_size(text_size);
        bounds = font.measure_text(self.text.as_bytes(), SkTextEncoding::UTF8, None).1;

        self.spec.text_scale_x = 1.0;
        self.spec.text_size = text_size;
        self.spec.bounds = bounds;

        // 横方向圧縮が必要な場合: 圧縮率の調整
        if bounds.width() > self.width {
            let mut i = self.width / bounds.width();
            loop {
                if i < 0.0 { break }

                font.set_scale_x(i);
                bounds = font.measure_text(self.text.as_bytes(), SkTextEncoding::UTF8, None).1;

                if bounds.width() <= self.width {
                    self.spec.bounds = bounds;
                    self.spec.text_scale_x = i;
                    break;
                }

                i -= 0.0001;
            }
        }
    }


    // Utils
    pub fn prepare_paint_for_measure(&self) -> SkPaint {
        let mut paint = SkPaint::default();
        paint.set_anti_alias(true);
        paint.set_color4f(SkColors::BLACK, None);
        return paint;
    }

    pub fn prepare_paint_for_draw(&self) -> SkPaint {
        let mut paint = SkPaint::default();
        paint.set_anti_alias(true);
        paint.set_color(self.color);
        return paint;
    }

    pub fn prepare_font_for_measure(&self) -> SkFont {
        let mut font = SkFont::default();
        font.set_typeface(&self.typeface);
        return font;
    }

    pub fn prepare_font_for_draw(&self) -> SkFont {
        let mut font = SkFont::default();
        font.set_typeface(&self.typeface);
        font.set_size(self.spec.text_size);
        font.set_scale_x(self.spec.text_scale_x);
        return font;
    }

    pub fn get_text_size(&self) -> SkScalar {
        return self.spec.text_size;
    }
}