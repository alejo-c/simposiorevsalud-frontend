use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::types::{AdminUpdateUserRequest, User};
use crate::utils::validate_password;

#[derive(Properties, PartialEq)]
pub struct AdminUpdateProps {
    pub user_id: String,
}

#[function_component(AdminUpdate)]
pub fn admin_update(props: &AdminUpdateProps) -> Html {
    let navigator = use_navigator().unwrap();
    let user_id = props.user_id.clone();

    // Form state
    let user = use_state(|| None::<User>);
    let email = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let identification = use_state(|| String::new());
    let password = use_state(|| String::new());
    let role = use_state(|| String::new());
    let hours = use_state(|| 0u8);
    let attendance = use_state(|| String::new());
    let message = use_state(|| String::new());
    let loading = use_state(|| true);

    // Fetch user data on mount
    {
        let user_id = user_id.clone();
        let user = user.clone();
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let role = role.clone();
        let hours = hours.clone();
        let attendance = attendance.clone();
        let message = message.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match ApiService::admin_get_user(&user_id).await {
                        Ok(user_data) => {
                            // Populate form with user data
                            email.set(user_data.email.clone());
                            full_name.set(user_data.full_name.clone());
                            identification.set(user_data.identification.clone());

                            // Handle role
                            match &user_data.role {
                                crate::types::UserRole::Simple(r) => {
                                    role.set(r.clone());
                                    hours.set(0);
                                }
                                crate::types::UserRole::Speaker { speaker } => {
                                    role.set("speaker".to_string());
                                    hours.set(speaker.hours);
                                }
                            }

                            attendance.set(user_data.attendance.clone());
                            user.set(Some(user_data));
                            loading.set(false);
                        }
                        Err(error) => {
                            message.set(format!("Error loading user: {}", error));
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_full_name_change = {
        let full_name = full_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            full_name.set(input.value());
        })
    };

    let on_identification_change = {
        let identification = identification.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            identification.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_role_change = {
        let role = role.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            role.set(select.value());
        })
    };

    let on_hours_change = {
        let hours = hours.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u8>() {
                hours.set(value);
            }
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
        let user_id = user_id.clone();
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let password = password.clone();
        let role = role.clone();
        let hours = hours.clone();
        let attendance = attendance.clone();
        let message = message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let user_id = user_id.clone();
            let email_val = (*email).clone();
            let full_name_val = (*full_name).clone();
            let identification_val = (*identification).clone();
            let password_val = (*password).clone();
            let role_val = (*role).clone();
            let hours_val = *hours;
            let attendance_val = (*attendance).clone();
            let message = message.clone();
            let navigator = navigator.clone();

            // Validate password if provided
            if !password_val.is_empty() && !validate_password(&password_val) {
                message.set("Contraseña no válida".to_string());
                return;
            }

            let data = AdminUpdateUserRequest {
                id: user_id,
                email: email_val,
                full_name: full_name_val,
                identification: identification_val,
                password: password_val,
                role: role_val,
                hours: hours_val,
                attendance: attendance_val,
            };

            spawn_local(async move {
                match ApiService::admin_update_user(data).await {
                    Ok(_) => {
                        navigator.push(&Route::AdminUsers);
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::AdminUsers);
        })
    };

    if *loading {
        html! {
            <div class="container">
                <div class="card">
                    <div class="spinner"></div>
                    <p>{"Cargando datos del usuario..."}</p>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="container">
                <div class="admin-header">
                    <h1>{"Editar Usuario"}</h1>
                </div>

                <div class="card">
                    <form onsubmit={on_submit}>
                        <div class="form-group">
                            <label for="email">{"Correo electrónico:"}</label>
                            <input
                                type="email"
                                id="email"
                                class="form-input"
                                required={true}
                                value={(*email).clone()}
                                onchange={on_email_change}
                            />
                        </div>

                        <div class="form-group">
                            <label for="full-name">{"Nombre completo:"}</label>
                            <input
                                type="text"
                                id="full-name"
                                class="form-input"
                                required={true}
                                value={(*full_name).clone()}
                                onchange={on_full_name_change}
                            />
                        </div>

                        <div class="form-group">
                            <label for="identification">{"Identificación:"}</label>
                            <input
                                type="text"
                                id="identification"
                                class="form-input"
                                required={true}
                                value={(*identification).clone()}
                                onchange={on_identification_change}
                            />
                        </div>

                        <div class="form-group">
                            <label for="password">{"Nueva contraseña (dejar vacío para no cambiar):"}</label>
                            <input
                                type="password"
                                id="password"
                                class="form-input"
                                value={(*password).clone()}
                                onchange={on_password_change}
                            />
                        </div>

                        <div class="form-group">
                            <label for="role">{"Rol:"}</label>
                            <select
                                id="role"
                                value={(*role).clone()}
                                onchange={on_role_change}
                            >
                                <option value="attendee">{"Asistente"}</option>
                                <option value="speaker">{"Ponente"}</option>
                                <option value="staff">{"Organizador"}</option>
                                <option value="webmaster">{"Administrador"}</option>
                            </select>
                        </div>

                        {if *role == "speaker" {
                            html! {
                                <div class="form-group">
                                    <label for="hours">{"Horas de ponencia:"}</label>
                                    <input
                                        type="number"
                                        id="hours"
                                        class="form-input"
                                        value={hours.to_string()}
                                        onchange={on_hours_change}
                                    />
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        <div class="form-group">
                            <label for="attendance">{"Tipo de asistencia:"}</label>
                            <select
                                id="attendance"
                                value={(*attendance).clone()}
                                onchange={on_attendance_change}
                            >
                                <option value="remote">{"Remota"}</option>
                                <option value="presential">{"Presencial"}</option>
                            </select>
                        </div>

                        {if !message.is_empty() {
                            html! {
                                <div id="message-span">{(*message).clone()}</div>
                            }
                        } else {
                            html! {}
                        }}

                        <div class="admin-actions">
                            <button type="submit" class="btn btn-success">
                                {"Guardar cambios"}
                            </button>
                            <button
                                type="button"
                                class="btn btn-outline"
                                onclick={on_cancel}
                            >
                                {"Cancelar"}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
