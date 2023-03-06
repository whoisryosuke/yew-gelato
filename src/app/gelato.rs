use std::{collections::HashMap, fs};

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

    let file_path = String::from("README.md");
    let contents =
        fs::read_to_string(file_path).expect("Couldn't read the file from the path provided");

    dbg!(contents);
    dbg!(props);
    // for value in props.iter() {
    //     dbg!(value);
    // }
}
