use std::collections::HashMap;

use stylist::Style;

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

pub fn generateResponsiveStyles<Props: std::fmt::Debug>(props: Props) {
    // let style_str = r#"
    //     background-color: red;

    //     .nested {
    //         background-color: blue;
    //         width: 100px
    //     }
    // "#;

    // let style = Style::new(style_str).expect("Failed to create style");

    // style.get_class_name()

    dbg!(props);
    // for value in props.iter() {
    //     dbg!(value);
    // }
}

type StringMap = HashMap<String, String>;
type IntegerMap = HashMap<String, i32>;
// Support for "object style" colors (e.g. colors.blue)
// @TODO: Maybe make this a struct with methods (like converting colors).
type ColorMap = HashMap<String, String>;
// Support for 1 level of nested colors (e.g. colors.blue.700)
type NestedColors = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone)]
struct Theme {
    media_queries: StringMap,
    colors: NestedColors,
    gradients: ColorMap,
    fonts: StringMap,
    font_sizes: Vec<i32>,
    font_weights: StringMap,
    line_heights: StringMap,
    space: Vec<i32>,
    radii: IntegerMap,
}

#[derive(Debug, Clone)]
pub struct ThemeBuilder {
    media_queries: Option<StringMap>,
    colors: Option<NestedColors>,
    gradients: Option<ColorMap>,
    fonts: Option<StringMap>,
    font_sizes: Option<Vec<i32>>,
    font_weights: Option<StringMap>,
    line_heights: Option<StringMap>,
    space: Option<Vec<i32>>,
    radii: Option<IntegerMap>,
}

impl ThemeBuilder {
    pub fn create(self) -> Theme {
        Theme {
            media_queries: self.media_queries.unwrap_or_default(),
            colors: self.colors.unwrap_or_default(),
            gradients: self.gradients.unwrap_or_default(),
            fonts: self.fonts.unwrap_or_default(),
            font_sizes: self.font_sizes.unwrap_or_default(),
            font_weights: self.font_weights.unwrap_or_default(),
            line_heights: self.line_heights.unwrap_or_default(),
            space: self.space.unwrap_or_default(),
            radii: self.radii.unwrap_or_default(),
        }
    }
}