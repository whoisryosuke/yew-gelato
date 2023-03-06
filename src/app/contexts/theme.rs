use std::collections::HashMap;
use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| {
            let mut colors = HashMap::new();
            colors.insert("text".to_string(), "#111212".to_string());
            colors.insert("background".to_string(), "#fff".to_string());
            colors.insert("primary".to_string(), "#005CDD".to_string());
            colors.insert("secondary".to_string(), "#6D59F0".to_string());
            colors.insert("muted".to_string(), "#f6f6f9".to_string());
            colors.insert("gray".to_string(), "#D3D7DA".to_string());
            colors.insert(
                "highlight".to_string(),
                "hsla(205, 100%, 40%, 0.125)".to_string(),
            );
            colors.insert("white".to_string(), "#FFF".to_string());
            colors.insert("black".to_string(), "#111212".to_string());
            Theme {
                font_color: "black".to_string(),
                background_color: "rgb(237, 244, 255)".to_string(),
                paper_color: "white".to_string(),
                space: vec![0, 4, 8, 16, 32, 64, 128, 256, 512],
                colors,
                media_queries: generate_media_queries(),
            }
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| {
            let mut colors = HashMap::new();
            colors.insert("text".to_string(), "#f6f6f9".to_string());
            colors.insert("background".to_string(), "#111212".to_string());
            colors.insert("primary".to_string(), "#005CDD".to_string());
            colors.insert("secondary".to_string(), "#6D59F0".to_string());
            colors.insert("muted".to_string(), "#f6f6f9".to_string());
            colors.insert("gray".to_string(), "#D3D7DA".to_string());
            colors.insert(
                "highlight".to_string(),
                "hsla(205, 100%, 40%, 0.125)".to_string(),
            );
            colors.insert("white".to_string(), "#FFF".to_string());
            colors.insert("black".to_string(), "#111212".to_string());
            Theme {
                font_color: "white".to_string(),
                background_color: "black".to_string(),
                paper_color: "rgb(50, 50, 50)".to_string(),
                space: vec![0, 4, 8, 16, 32, 64, 128, 256, 512],
                colors,
                media_queries: generate_media_queries(),
            }
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }
}

const BREAKPOINTS: [&str; 5] = [
    "320px",  // mobile
    "768px",  // tablet
    "992px",  // computer
    "1200px", // desktop
    "1920px", // widescreen
];

// fn generate_media_queries() -> MediaQueries {
//     MediaQueries {
//         mobile: format!("@media screen and (min-width: ${})", BREAKPOINTS[0]),
//         tablet: format!("@media screen and (min-width: ${})", BREAKPOINTS[1]),
//         computer: format!("@media screen and (min-width: ${})", BREAKPOINTS[2]),
//         desktop: format!("@media screen and (min-width: ${})", BREAKPOINTS[3]),
//         widescreen: format!("@media screen and (min-width: ${})", BREAKPOINTS[4]),
//     }
// }

type MediaQueriesArray = [String; 5];

// fn generate_media_queries() -> MediaQueriesArray {
//     [
//         format!("@media screen and (min-width: ${})", BREAKPOINTS[0]),
//         format!("@media screen and (min-width: ${})", BREAKPOINTS[1]),
//         format!("@media screen and (min-width: ${})", BREAKPOINTS[2]),
//         format!("@media screen and (min-width: ${})", BREAKPOINTS[3]),
//         format!("@media screen and (min-width: ${})", BREAKPOINTS[4]),
//     ]
// }

fn generate_media_queries() -> MediaQueriesArray {
    [
        BREAKPOINTS[0].to_string(),
        BREAKPOINTS[1].to_string(),
        BREAKPOINTS[2].to_string(),
        BREAKPOINTS[3].to_string(),
        BREAKPOINTS[4].to_string(),
    ]
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,

    pub paper_color: String,
    pub space: Vec<i32>,
    pub colors: HashMap<String, String>,
    pub media_queries: MediaQueriesArray,
}

#[derive(Debug, Clone)]
pub(crate) struct MediaQueries {
    mobile: String,
    tablet: String,
    computer: String,
    desktop: String,
    widescreen: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Colors {
    text: String,
    background: String,
    primary: String,
    secondary: String,
    muted: String,
    gray: String,
    highlight: String,
    white: String,
    black: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub(crate) fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| ThemeKind::Light);

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
