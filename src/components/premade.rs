use crate::types::{Game, PremadeGamesInfo, ResponsePremadeGame};
use leptos::prelude::*;
use reqwest_wasm::Client;
use thiserror::Error;
use base64::prelude::*;

use super::{GameList, Spinner};

async fn get_all_games() -> Result<Vec<Game>, Error> {
    let client = Client::new();
    let request = client.get("https://motionvid.pl/Bingo/list_premade.php");
    let result = request.send().await?;
    let result_text = result.text().await?;
    let games = serde_json::from_str::<PremadeGamesInfo>(&result_text)?.games;

    let mut ret = vec![];
    for game in games {
        let request = client.get(format!("https://motionvid.pl/Bingo/get_premade.php?game={}", game));
        let result = request.send().await?;
        let result_text = result.text().await?;
        let game_b64enc: ResponsePremadeGame = serde_json::from_str(&result_text).unwrap();
        let game = String::from_utf8(BASE64_STANDARD.decode(game_b64enc.game).unwrap()).unwrap();

        ret.push(serde_json::from_str(&game)?);
    }

    Ok(ret)
}

#[component]
pub fn Premade() -> impl IntoView {
    let games = LocalResource::new(move || get_all_games());

    view! {
        <Suspense fallback=move || {
            view! { <Spinner /> }
        }>
            {move || Suspend::new(async move {
                let games = games.await;

                view! { <GameList games=games.unwrap() /> }
            })}
        </Suspense>
    }
}
