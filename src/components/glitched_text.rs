use leptos::prelude::*;

turf::style_sheet!("src/styles/glitched_text.scss");

#[component]
pub fn GlitchedText(text: &'static str) -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>

        <div class=ClassName::GLITCHEDCONTAINER>
            <span style="--index: 0;">{text}</span>
            <span style="--index: 1;">{text}</span>
            <span style="--index: 2;">{text}</span>
        </div>
    }
}
