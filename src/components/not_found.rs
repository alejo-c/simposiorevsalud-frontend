use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div class="container text-center">
            <div class="card" style="max-width: 600px; margin: 0 auto;">
                <h1 class="glow-text" style="font-size: 6rem; margin-bottom: 0;">{"404"}</h1>
                <h2>{"Página no encontrada"}</h2>
                <p>{"Lo sentimos, la página que buscas no existe."}</p>
                <button class="btn mt-3" onclick={on_home_click}>
                    {"Volver al inicio"}
                </button>
            </div>
        </div>
    }
}
