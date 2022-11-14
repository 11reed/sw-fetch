use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct SwApi {
    name: String,
    birth_year: String,
    home_world: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
}

async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(sw_data) = data.json::<T>().await {
            Ok(sw_data)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

async fn fetch_sw(sw_data: String) -> Result<SwApi, Error> {
    fetch::<SwApi>(format!("https://swapi.dev/api/{}", sw_data)).await
}


#[function_component(SwFetch)]
fn sw_fetch() -> Html {
    let state = use_async(async move { fetch_sw("people/1/".to_string()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    /*
    
    
    */

    html! {
        <div>
            <button {onclick} disabled={state.loading}>{ "Load Star Wars Information: " }</button>
            <p>
                {
                    if state.loading {
                        html! { "Loading..." }
                    } else {
                        html! {}
                    }
                }
            </p>
            {
                if let Some(sw_character) = &state.data {
                    html! {
                        <>
                            <p>{ "Name: " }<b>{ &sw_character.name }</b></p>
                            <p>{ "Birth Year: " }<b>{ &sw_character.birth_year }</b></p>
                            <p>{ "Home World: " }<b>{ &sw_character.home_world }</b></p>
                        </>
                        }
                } else {
                    html! {}
                }
            }
            <p>
                {
                    if let Some(error) = &state.error {
                        match error {
                            Error::DeserializeError => html! { "DeserializeError" },
                            Error::RequestError => html! { "RequestError" },
                        }
                    } else {
                        html! {}
                    }
                }
            </p>
        </div>
    }
}

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <div class={classes!("container")}>
            <div class={classes!("info")}>
                <SwFetch />
            </div>
        </div>
    }
}