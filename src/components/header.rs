use leptos::prelude::*;

use super::GlitchedText;

turf::style_sheet!("src/styles/header.scss");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>

        <header>
            <a href="/bingo-leptos" class=ClassName::SIDENAV>
                "Oficjalne"
            </a>
            <h1>
                <a href="/bingo-leptos" id="main-text">
                    <GlitchedText text="BINGO" />
                </a>
            </h1>
            <a href="/bingo-leptos/custom" class=ClassName::SIDENAV>
                "Customy"
            </a>
        </header>
    }
}
