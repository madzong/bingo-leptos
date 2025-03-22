use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use super::{Custom, Header, Premade};

turf::style_sheet!("src/styles/root.scss");

/// Main application component
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>

        <Router>
            <Header />
            <main>
                <Routes fallback=|| "Route not found.">
                    <Route path=path!("/bingo-leptos") view=Premade />
                    <Route path=path!("/bingo-leptos/custom") view=Custom />
                </Routes>
            </main>
        </Router>
    }
}
