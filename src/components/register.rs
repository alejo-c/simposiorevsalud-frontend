use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::services::api::ApiService;
use crate::types::RegisterRequest;
use crate::utils::validate_password;

#[function_component(Register)]
pub fn register() -> Html {
    let email = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let identification = use_state(|| String::new());
    let password = use_state(|| String::new());
    let repeated_password = use_state(|| String::new());
    let attendance = use_state(|| "remote".to_string());
    let message = use_state(|| String::new());

    // Debug button handlers
    let on_test_connection = {
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();
            spawn_local(async move {
                match ApiService::test_api_connection().await {
                    Ok(result) => {
                        log::info!("Connection test: {}", result);
                        message.set(format!("Connection test: {}", result));
                    }
                    Err(error) => {
                        log::error!("Connection test failed: {}", error);
                        message.set(format!("Connection test failed: {}", error));
                    }
                }
            });
        })
    };

    let on_test_options = {
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();
            spawn_local(async move {
                match ApiService::test_options_request().await {
                    Ok(result) => {
                        log::info!("OPTIONS test: {}", result);
                        message.set(format!("OPTIONS test: {}", result));
                    }
                    Err(error) => {
                        log::error!("OPTIONS test failed: {}", error);
                        message.set(format!("OPTIONS test failed: {}", error));
                    }
                }
            });
        })
    };

    let on_email_change = {
        let email = email.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            message.set(String::new());
        })
    };

    let on_full_name_change = {
        let full_name = full_name.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            full_name.set(input.value());
            message.set(String::new());
        })
    };

    let on_identification_change = {
        let identification = identification.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            identification.set(input.value());
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

    let on_repeated_password_change = {
        let repeated_password = repeated_password.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            repeated_password.set(input.value());
            message.set(String::new());
        })
    };

    let on_attendance_change = {
        let attendance = attendance.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            attendance.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        let attendance = attendance.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
            let full_name_val = (*full_name).clone();
            let identification_val = (*identification).clone();
            let password_val = (*password).clone();
            let repeated_password_val = (*repeated_password).clone();
            let attendance_val = (*attendance).clone();
            let message = message.clone();

            if !validate_password(&password_val) || !validate_password(&repeated_password_val) {
                message.set("Contraseña no valida".to_string());
                return;
            }

            if password_val != repeated_password_val {
                message.set("Contraseñas no coinciden".to_string());
                return;
            }

            let data = RegisterRequest {
                email: email_val,
                full_name: full_name_val,
                identification: identification_val,
                password: password_val,
                role: "attendee".to_string(),
                presentation: String::new(),
                attendance: attendance_val,
            };

            spawn_local(async move {
                match ApiService::register(data).await {
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Registro exitoso".to_string());
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
            <h1>{"Registro"}</h1>

            <section>
                // Debug buttons - remove these after fixing the issue
                <div style="margin-bottom: 20px; padding: 10px; background-color: #f0f0f0; border-radius: 5px;">
                    <h3>{"Debug Tests"}</h3>
                    <button
                        type="button"
                        onclick={on_test_connection}
                        style="margin-right: 10px; background-color: #007bff; color: white; padding: 5px 10px; border: none; border-radius: 3px;"
                    >
                        {"Test Connection"}
                    </button>
                    <button
                        type="button"
                        onclick={on_test_options}
                        style="background-color: #28a745; color: white; padding: 5px 10px; border: none; border-radius: 3px;"
                    >
                        {"Test OPTIONS"}
                    </button>
                </div>

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
                        <label for="full-name-input">{"Nombre completo:"}</label>
                        <input
                            type="text"
                            id="full-name-input"
                            class="form-input"
                            required={true}
                            value={(*full_name).clone()}
                            onchange={on_full_name_change}
                        />
                    </div>
                    <div class="form-group">
                        <label for="id-input">{"Documento de identificación:"}</label>
                        <input
                            type="text"
                            id="id-input"
                            class="form-input"
                            required={true}
                            value={(*identification).clone()}
                            onchange={on_identification_change}
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
                    <div class="form-group">
                        <label for="repeat-password-input">{"Repita contraseña:"}</label>
                        <input
                            type="password"
                            id="repeat-password-input"
                            class="form-input"
                            required={true}
                            minlength="8"
                            value={(*repeated_password).clone()}
                            onchange={on_repeated_password_change}
                        />
                    </div>
                    <div class="form-group">
                        <label for="attendance-select">{"Tipo de asistencia:"}</label>
                        <select id="attendance-select" value={(*attendance).clone()} onchange={on_attendance_change}>
                            <option value="remote">{"Remota"}</option>
                            <option value="presential">{"Presencial"}</option>
                        </select>
                    </div>

                    <div>
                        <span id="message-span">{(*message).clone()}</span>
                    </div>

                    <button type="submit" id="register-btn">{"Registrarse"}</button>
                </form>
            </section>
        </>
    }
}
