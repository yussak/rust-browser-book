use alloc::string::String;

use crate::error::Error;

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

#[derive(Debug, Clone, PartialEq)]
// red,blueなどの名前や#fffなどのコードを持つ
pub struct Color {
    name: Option<String>,
    code: String,
}

impl Color {
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let code = match name {
            "black" => "#000000".to_string(),
            "silver" => "#c0c0c0".to_string(),
            "gray" => "#808080".to_string(),
            "white" => "#ffffff".to_string(),
            "maroon" => "#800000".to_string(),
            "red" => "#ff0000".to_string(),
            "purple" => "#800080".to_string(),
            "fuchsia" => "#ff00ff".to_string(),
            "green" => "#008000".to_string(),
            "lime" => "#00ff00".to_string(),
            "olive" => "#808000".to_string(),
            "yellow" => "#ffff00".to_string(),
            "navy" => "#000080".to_string(),
            "blue" => "#0000ff".to_string(),
            "teal" => "#008080".to_string(),
            "aqua" => "#00ffff".to_string(),
            "orange" => "#ffa500".to_string(),
            "lightgray" => "#d3d3d3".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color name {:?} is not supported yet",
                    name
                )));
            }
        };

        Ok(Self {
            name: Some(name.to_string()),
            code,
        })
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        if code.chars().nth(0) != Some("#") || code.len() != 7 {
            return Err(Error::UnexpectedInput(format!(
                "invalid color code {}",
                code
            )));
        }

        let name = match code {
            "#000000" => "black".to_string(),
            "#c0c0c0" => "silver".to_string(),
            "#808080" => "gray".to_string(),
            "#ffffff" => "white".to_string(),
            "#800000" => "maroon".to_string(),
            "#ff0000" => "red".to_string(),
            "#800080" => "purple".to_string(),
            "#ff00ff" => "fuchsia".to_string(),
            "#008000" => "green".to_string(),
            "#00ff00" => "lime".to_string(),
            "#808000" => "olive".to_string(),
            "#ffff00" => "yellow".to_string(),
            "#000080" => "navy".to_string(),
            "#0000ff" => "blue".to_string(),
            "#008080" => "teal".to_string(),
            "#00ffff" => "aqua".to_string(),
            "#ffa500" => "orange".to_string(),
            "#d3d3d3" => "lightgray".to_string(),

            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color code {:?} is not supported yet",
                    name =
                )));
            }
        };

        Ok(Self {
            name: Some(name),
            code: code.to_string(),
        })
    }

    pub fn white() -> Self {
        Self {
            name: Some("white".to_string()),
            code: "#ffffff".to_string(),
        }
    }

    pub fn black() -> Self {
        Self {
            name: Some("black".to_string()),
            code: "#000000".to_string(),
        }
    }

    pub fn code_u32(&self) -> u32 {
        u32::from_str_radix(self.code.trim_start_matches("#"), 16).unwrap()
    }
}
