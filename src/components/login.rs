use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::services::auth::AuthService;
use crate::types::LoginRequest;
use crate::utils::validate_password;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let message = use_state(|| String::new());

    let on_email_change = {
        let email = email.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            message.set(String::new());
        })
    };

    let on_password_change = {
        let password = password.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
            message.set(String::new());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let message = message.clone();
            let navigator = navigator.clone();

            if !validate_password(&password_val) {
                message.set("Contraseña no valida".to_string());
                return;
            }

            let data = LoginRequest {
                email: email_val,
                password: password_val,
            };

            spawn_local(async move {
                match ApiService::login(data).await {
                    Ok(token) => {
                        AuthService::set_token(token);
                        navigator.push(&Route::Profile);
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
            <h1>{"Ingreso"}</h1>
            <section>
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="email-input">{"Correo electrónico:"}</label>
                        <input
                            type="email"
                            id="email-input"
                            class="form-input"
                            required={true}
                            minlength="5"
                            value={(*email).clone()}
                            onchange={on_email_change}
                        />
                    </div>
                    <div class="form-group">
                        <label for="password-input">{"Contraseña:"}</label>
                        <input
                            type="password"
                            id="password-input"
                            class="form-input"
                            required={true}
                            minlength="8"
                            value={(*password).clone()}
                            onchange={on_password_change}
                        />
                    </div>

                    <div>
                        <span id="message-span">{(*message).clone()}</span>
                    </div>
                    <button type="submit" id="login-btn">{"Ingresar"}</button>
                </form>
            </section>
        </>
    }
}
