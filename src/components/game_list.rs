use crate::types::Game;
use leptos::prelude::*;

turf::style_sheet!("src/styles/game_list.scss");

#[component]
pub fn GameList(games: Vec<Game>) -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>

        <div class=ClassName::GAMELIST>
            {games.iter().map(|g| view! { <p>{g.name.clone()}</p> }).collect::<Vec<_>>()}
        </div>
    }
}
