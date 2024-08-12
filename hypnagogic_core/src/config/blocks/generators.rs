use serde::{Deserialize, Serialize};

use crate::generation::rect::{Border, BorderStyle};
use crate::generation::text::Alignment;
use crate::util::color::Color;
use crate::util::icon_ops::pick_contrasting_colors;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    #[default]
    BottomRight,
    Center,
}

fn white() -> Color {
    Color::new(255, 255, 255, 255)
}

fn black() -> Color {
    Color::new(0, 0, 0, 255)
}

fn bottom_right() -> Position {
    Position::BottomRight
}

fn default_dim() -> u32 {
    32
}

#[allow(clippy::unnecessary_wraps)] // wrap is needed for serde default
fn default_outer_border() -> Option<Border> {
    Some(Border {
        style: BorderStyle::Solid,
        color: Color::new(0, 0, 0, 255),
    })
}

fn default_alignment() -> Alignment {
    Alignment::Right
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MapIcon {
    pub icon_state_name: String,
    #[serde(default)]
    pub automatic: bool,
    #[serde(default = "default_dim")]
    pub height: u32,
    #[serde(default = "default_dim")]
    pub width: u32,
    #[serde(default = "white")]
    pub base_color: Color,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default = "black")]
    pub text_color: Color,
    #[serde(default = "bottom_right")]
    pub text_position: Position,
    #[serde(default = "default_alignment")]
    pub text_alignment: Alignment,
    #[serde(default)]
    pub inner_border: Option<Border>,
    #[serde(default = "default_outer_border")]
    pub outer_border: Option<Border>,
}

impl Default for MapIcon {
    fn default() -> Self {
        Self {
            icon_state_name: "map_icon".to_string(),
            automatic: false,
            height: 32,
            width: 32,
            base_color: Color::new(255, 255, 255, 255),
            text: Some("DEF".to_string()),
            text_color: Color::new(0, 0, 0, 255),
            text_position: Position::BottomRight,
            text_alignment: Alignment::Right,
            inner_border: None,
            outer_border: Some(Border {
                style: BorderStyle::Solid,
                color: Color::new(0, 0, 0, 255),
            }),
        }
    }
}

impl MapIcon {
    pub fn gen_colors(&mut self, colors: &[Color]) {
        if !self.automatic {
            return;
        }
        let sorted_colors = pick_contrasting_colors(colors);
        self.base_color = sorted_colors.0;
        self.text_color = sorted_colors.1;
        self.outer_border = Some(Border {
            style: BorderStyle::Solid,
            color: sorted_colors.1,
        });
    }
}
