use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::api::ApiService;

#[function_component(Certificates)]
pub fn certificates() -> Html {
    let message = use_state(|| String::new());

    let on_generate_horizontal = {
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();

            spawn_local(async move {
                match ApiService::generate_horizontal_cert().await {
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Certificado horizontal generado exitosamente".to_string());
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    let on_generate_vertical = {
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();

            spawn_local(async move {
                match ApiService::generate_vertical_cert().await {
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Certificado vertical generado exitosamente".to_string());
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    html! {
        <>
            <h1>{"Descargar certificados"}</h1>
            <article>
                <section>
                    <figure>
                        <img src="/img/horiz_cert_example.png" alt="Horizontal certificate example" />
                    </figure>
                    <p>{"Horizontal certificate"}</p>
                    <button id="horiz-cert-btn" class="btn" onclick={on_generate_horizontal}>
                        {"Generate"}
                    </button>
                </section>

                <section>
                    <figure>
                        <img src="/img/vert_cert_example.png" alt="Vertical certificate example" />
                    </figure>
                    <p>{"Vertical certificate"}</p>
                    <button id="vert-cert-btn" class="btn" onclick={on_generate_vertical}>
                        {"Generate"}
                    </button>
                </section>

                <div>
                    <span id="message-span">{(*message).clone()}</span>
                </div>
            </article>
        </>
    }
}
