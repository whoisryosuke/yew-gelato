use log::info;
use std::collections::HashMap;

use super::contexts::Theme;
use stylist::{style, Style};

pub trait ResponsiveStyleProps {}

struct MediaQueries {}

// Props can be an array of strings (e.g.  flexDirection: ["row", "column"],)
// or a HashMap that represents "look up key" + CSS value
enum ResponsiveProp {
    Strings(Vec<String>),
    Keys(HashMap<String, String>),
}

struct ResponsiveStyleConfig {
    conditions: MediaQueries,
    default_condition: String,
    properties: HashMap<String, ResponsiveProp>,
}

// fn createResponsiveStyleProps(config: ResponsiveStyleConfig) -> Fn {
//     // Return a function the user can use in their component
//     // Function should return CSS class for the appropriate responsive styles
//     return generateResponsiveStyles;
// }

pub(crate) fn generateResponsiveStyles(theme: &Theme, margin_key: usize) -> Style {
    let margin = if theme.space.len() >= margin_key {
        theme.space[margin_key]
    } else {
        0
    };

    let second_class = style!(
        r#"
            margin-top: ${margin}px;
        "#,
        margin = margin
    )
    .expect("Failed to mount style");

    second_class
}

pub(crate) fn generate_color_styles(theme: &Theme, color_key: String) -> Style {
    let color = if theme.colors.contains_key(&color_key) {
        theme
            .colors
            .get(&color_key)
            .expect("Couldn't find key")
            .clone()
    } else {
        "#000".to_string()
    };

    let style = style!(
        r#"
            color: ${color};
        "#,
        color = color
    )
    .expect("Failed to mount style");

    style
}

// type StringMap = HashMap<String, String>;
// type IntegerMap = HashMap<String, i32>;
// // Support for "object style" colors (e.g. colors.blue)
// // @TODO: Maybe make this a struct with methods (like converting colors).
// type ColorMap = HashMap<String, String>;
// // Support for 1 level of nested colors (e.g. colors.blue.700)
// type NestedColors = HashMap<String, HashMap<String, String>>;

// #[derive(Debug, Clone)]
// pub struct Theme {
//     media_queries: StringMap,
//     colors: NestedColors,
//     gradients: ColorMap,
//     fonts: StringMap,
//     font_sizes: Vec<i32>,
//     font_weights: StringMap,
//     line_heights: StringMap,
//     space: Vec<i32>,
//     radii: IntegerMap,
// }

// #[derive(Debug, Clone)]
// pub struct ThemeBuilder {
//     media_queries: Option<StringMap>,
//     colors: Option<NestedColors>,
//     gradients: Option<ColorMap>,
//     fonts: Option<StringMap>,
//     font_sizes: Option<Vec<i32>>,
//     font_weights: Option<StringMap>,
//     line_heights: Option<StringMap>,
//     space: Option<Vec<i32>>,
//     radii: Option<IntegerMap>,
// }

// impl ThemeBuilder {
//     pub fn create(self) -> Theme {
//         Theme {
//             media_queries: self.media_queries.unwrap_or_default(),
//             colors: self.colors.unwrap_or_default(),
//             gradients: self.gradients.unwrap_or_default(),
//             fonts: self.fonts.unwrap_or_default(),
//             font_sizes: self.font_sizes.unwrap_or_default(),
//             font_weights: self.font_weights.unwrap_or_default(),
//             line_heights: self.line_heights.unwrap_or_default(),
//             space: self.space.unwrap_or_default(),
//             radii: self.radii.unwrap_or_default(),
//         }
//     }
// }
