use log::info;
use stylist::{
    style,
    yew::{styled_component, Global},
};
use yew::prelude::*;

use crate::app::gelato::{generateResponsiveStyles, generate_color_styles};
mod contexts;
mod gelato;

use contexts::{use_theme, ThemeKind, ThemeProvider};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub margin: usize,
    pub color: String,
}

#[styled_component]
pub fn Inside(props: &Props) -> Html {
    let theme = use_theme();

    // info!("Props {:?}", props);
    // info!("Theme {:?}", theme.kind());
    // info!("Theme Styles {:?}", theme.kind().current());

    let first_class = style!(
        r#"
            width: 200px;
            height: 200px;
            border-radius: 5px;
            background: black;
            padding: 15px;
            box-sizing: border-box;
            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            color: white;
        "#
    )
    .expect("Failed to mount style");

    // let margin = generateResponsiveStyles(theme.kind().current(), props.margin);
    let color = generate_color_styles(theme.kind().current(), props.color.clone());

    html! {
        <div class={format!("{} {}", first_class.get_class_name(), color.get_class_name())}>
            {"The quick brown fox jumps over the lazy dog"}
        </div>
    }
}

#[styled_component]
pub fn Button(props: &Props) -> Html {
    let theme = use_theme();

    // info!("Props {:?}", props);
    // info!("Theme {:?}", theme.kind());
    // info!("Theme Styles {:?}", theme.kind().current());

    let first_class = style!(
        r#"
            width: 200px;
            height: 100px;
            border-radius: 5px;
            background: #333;
            padding: 15px;
            box-sizing: border-box;
            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            color: white;
        "#
    )
    .expect("Failed to mount style");

    // let margin = generateResponsiveStyles(theme.kind().current(), props.margin);
    let color = generate_color_styles(theme.kind().current(), props.color.clone());

    html! {
        <button class={format!("{} {}", first_class.get_class_name(), color.get_class_name())}>
            {"The quick brown fox jumps over the lazy dog"}
        </button>
    }
}

#[styled_component]
pub fn App() -> Html {
    html! {
        <>
            // Global Styles can be applied with <Global /> component.
            <Global css={css!(r#"
                    html, body {
                        font-family: sans-serif;
                        padding: 0;
                        margin: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;
                        background-color: rgb(237, 244, 255);
                    }
                "#)} />
            <h1>{"Yew Integration"}</h1>
            <div class={css!(r#"
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                height: 500px;
                width: 500px;
                border-radius: 5px;
                display: flex;
                justify-content: space-around;
                align-items: center;
                padding: 15px;
                box-sizing: border-box;
                flex-direction: column;
                background-color: white;
            "#)} id="yew-sample-content">
                {"The quick brown fox jumps over the lazy dog"}
                <Inside margin={4} color="text" />
                <Inside margin={4} color="text" />
                <Inside margin={4} color="primary" />
                <Button margin={4} color="text" />
                <Button margin={4} color="primary" />
            </div>
        </>
    }
}

#[styled_component]
pub fn Root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}
