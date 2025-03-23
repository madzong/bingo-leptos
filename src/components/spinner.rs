use leptos::prelude::*;

turf::style_sheet!("src/styles/spinner.scss");

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>

        <div class=ClassName::LOADER />
    }
}
