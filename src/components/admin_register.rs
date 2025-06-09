use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::services::api::ApiService;
use crate::types::RegisterRequest;
use crate::utils::validate_password;

#[function_component(AdminRegister)]
pub fn admin_register() -> Html {
    let email = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let identification = use_state(|| String::new());
    let password = use_state(|| String::new());
    let repeated_password = use_state(|| String::new());
    let role = use_state(|| "attendee".to_string());
    let presentation = use_state(|| String::new());
    let attendance = use_state(|| "remote".to_string());
    let message = use_state(|| String::new());

    // Test data population (similar to the original JS test function)
    {
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        let role = role.clone();
        let presentation = presentation.clone();
        let attendance = attendance.clone();

        use_effect_with_deps(
            move |_| {
                // Populate test data
                email.set("test@gmail.com".to_string());
                full_name.set("Test Example".to_string());
                identification.set("1234567890".to_string());
                password.set("T3stexampl*".to_string());
                repeated_password.set("T3stexampl*".to_string());
                role.set("attendee".to_string());
                presentation.set(String::new());
                attendance.set("remote".to_string());
                || ()
            },
            (),
        );
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

    let on_role_change = {
        let role = role.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            role.set(select.value());
        })
    };

    let on_presentation_change = {
        let presentation = presentation.clone();
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            presentation.set(input.value());
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
        let full_name = full_name.clone();
        let identification = identification.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        let role = role.clone();
        let presentation = presentation.clone();
        let attendance = attendance.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
            let full_name_val = (*full_name).clone();
            let identification_val = (*identification).clone();
            let password_val = (*password).clone();
            let repeated_password_val = (*repeated_password).clone();
            let role_val = (*role).clone();
            let presentation_val = (*presentation).clone();
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
                role: role_val,
                presentation: presentation_val,
                attendance: attendance_val,
            };

            spawn_local(async move {
                match ApiService::register(data).await {
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Usuario registrado exitosamente".to_string());
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
            <h1>{"Registro de usuarios"}</h1>

            <section>
                <form id="register-form" onsubmit={on_submit}>
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
                        <label for="role-select">{"Rol:"}</label>
                        <select id="role-select" value={(*role).clone()} onchange={on_role_change}>
                            <option value="attendee">{"Asistente"}</option>
                            <option value="speaker">{"Ponente"}</option>
                            <option value="staff">{"Organizador"}</option>
                        </select>
                    </div>

                    <div class={if *role != "speaker" { "form-group hidden" } else { "form-group" }}>
                        <label for="hours-input">{"Cantidad de horas de la ponencia (0 por defecto):"}</label>
                        <input
                            type="number"
                            id="hours-input"
                            class="form-input"
                            value={presentation.to_string()}
                            onchange={on_presentation_change}
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
