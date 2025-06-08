use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::services::api::ApiService;
use crate::types::UpdateUserRequest;
use crate::utils::validate_password;

#[function_component(Profile)]
pub fn profile() -> Html {
    let email = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let identification = use_state(|| String::new());
    let password = use_state(|| String::new());
    let repeated_password = use_state(|| String::new());
    let attendance = use_state(|| "remote".to_string());
    let message = use_state(|| String::new());

    // Fill form with existing user data on mount
    {
        let full_name = full_name.clone();
        let identification = identification.clone();
        use_effect_with((), move |_| {
            // In a real app, you'd fetch current user data here
            // For now, we'll just set empty values
            full_name.set(String::new());
            identification.set(String::new());
            || ()
        });
    }

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
            let select: HtmlSelectElement = e.target_unchecked_into();
            attendance.set(select.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        let attendance = attendance.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
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

            let data = UpdateUserRequest {
                email: email_val,
                password: password_val,
                attendance: attendance_val,
            };

            spawn_local(async move {
                match ApiService::update_user(data).await {
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Perfil actualizado exitosamente".to_string());
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
            <h1>{"Perfil"}</h1>
            <section>
                <form id="profile-form" onsubmit={on_submit}>
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
                            disabled={true}
                            value={(*full_name).clone()}
                        />
                    </div>
                    <div class="form-group">
                        <label for="id-input">{"Documento de identificación:"}</label>
                        <input
                            type="text"
                            id="id-input"
                            class="form-input"
                            disabled={true}
                            value={(*identification).clone()}
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

                    <button type="submit" id="register-btn">{"Actualizar"}</button>
                </form>
            </section>
        </>
    }
}
