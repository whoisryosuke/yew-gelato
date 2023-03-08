use log::info;
use std::collections::HashMap;

use super::contexts::Theme;
use stylist::{style, yew::use_media_query, Style};

pub(crate) fn generateResponsiveStyles(theme: &Theme, values: Vec<i32>) -> String {
    let mut styles: Vec<String> = Vec::new();

    for (index, prop_value) in values.iter().enumerate() {
        let key = *prop_value as usize;
        // Check if we even need to print the style based on current breakpoint
        let margin = if theme.space.len() > key {
            theme.space[key]
        } else {
            prop_value.clone()
        };

        let css_property = style!(
            r#"
            @media screen and (min-width: ${breakpoint}) {
                margin: ${margin}px;
            }
        "#,
            margin = margin,
            breakpoint = theme.media_queries[index]
        )
        .expect("");

        styles.push(css_property.get_class_name().to_string());
    }

    let combined_styles = styles.join(" ");

    combined_styles
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
