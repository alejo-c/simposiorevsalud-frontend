mod components;
mod routes;
mod services;
mod types;
mod utils;

use routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logger");

    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
