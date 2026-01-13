#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<FontSize>,
    text_decoration: Option<TextDecoration>,
    height: Option<i64>,
    width: Option<i64>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            display: None,
            font_size: None,
            text_decoration: None,
            height: None,
            width: None,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = Some(color)
    }

    pub fn background_color(&self) -> Color {
        self.background_color
            .clone()
            .expect("failed to access CSS property: background_color")
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color)
    }

    pub fn color(&self) -> Color {
        self.color
            .clone()
            .expect("failed to access CSS property: color")
    }

    pub fn set_display(&mut self, display: DisplayType) {
        self.display = Some(display)
    }

    pub fn display(&self) -> DisplayType {
        self.display
            .clone()
            .expect("failed to access CSS property: display")
    }

    pub fn set_font_size(&mut self, font_size: FontSize) {
        self.font_size = Some(font_size)
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
            .clone()
            .expect("failed to access CSS property: font_size")
    }

    pub fn set_text_decoration(&mut self, text_decoration: TextDecoration) {
        self.text_decoration = Some(text_decoration)
    }

    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .clone()
            .expect("failed to access CSS property: text_decoration")
    }

    pub fn set_height(&mut self, height: i64) {
        self.height = Some(height)
    }

    pub fn height(&self) -> i64 {
        self.height
            .clone()
            .expect("failed to access CSS property: height")
    }

    pub fn set_width(&mut self, width: i64) {
        self.width = Some(width)
    }

    pub fn width(&self) -> i64 {
        self.width
            .clone()
            .expect("failed to access CSS property: width")
    }
}
