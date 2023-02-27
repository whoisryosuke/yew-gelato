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
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "black".to_string(),
            background_color: "rgb(237, 244, 255)".to_string(),
            paper_color: "white".to_string(),
            space: vec![0, 4, 8, 16, 32, 64, 128, 256, 512],
            colors: Colors {
                text: "#111212".to_string(),
                background: "#fff".to_string(),
                primary: "#005CDD".to_string(),
                secondary: "#6D59F0".to_string(),
                muted: "#f6f6f9".to_string(),
                gray: "#D3D7DA".to_string(),
                highlight: "hsla(205, 100%, 40%, 0.125)".to_string(),
                white: "#FFF".to_string(),
                black: "#111212".to_string(),
            },
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "white".to_string(),
            background_color: "black".to_string(),
            paper_color: "rgb(50, 50, 50)".to_string(),
            space: vec![0, 4, 8, 16, 32, 64, 128, 256, 512],
            // colors: HashMap::new().insert(k, v)
            colors: Colors {
                text: "#f6f6f9".to_string(),
                background: "#111212".to_string(),
                primary: "#005CDD".to_string(),
                secondary: "#6D59F0".to_string(),
                muted: "#f6f6f9".to_string(),
                gray: "#D3D7DA".to_string(),
                highlight: "hsla(205, 100%, 40%, 0.125)".to_string(),
                white: "#FFF".to_string(),
                black: "#111212".to_string(),
            },
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,

    pub paper_color: String,
    pub space: Vec<i32>,
    pub colors: HashMap<String, String>,
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
