use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::types::{AdminUpdateUserRequest, DeleteUserRequest, User, UserRole};
use crate::utils::validate_password;

#[function_component(AdminUsers)]
pub fn admin_users() -> Html {
    let navigator = use_navigator().unwrap();
    let users = use_state(|| Vec::<User>::new());
    let message = use_state(|| String::new());
    let show_update_modal = use_state(|| false);

    // Update form state
    let sid_input = use_state(|| String::new());
    let email = use_state(|| String::new());
    let full_name = use_state(|| String::new());
    let identification = use_state(|| String::new());
    let password = use_state(|| String::new());
    let repeated_password = use_state(|| String::new());
    let role = use_state(|| "attendee".to_string());
    let hours = use_state(|| 0u32);
    let attendance = use_state(|| "remote".to_string());

    // Load users on component mount
    {
        let users = users.clone();
        let message = message.clone();
        use_effect_with((), move |_| {
            let users = users.clone();
            let message = message.clone();
            spawn_local(async move {
                match ApiService::admin_get_users().await {
                    Ok(user_list) => {
                        users.set(user_list);
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
            || ()
        });
    }

    let on_register_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::AdminRegister);
        })
    };

    let on_reload_click = {
        let users = users.clone();
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let users = users.clone();
            let message = message.clone();
            spawn_local(async move {
                match ApiService::admin_get_users().await {
                    Ok(user_list) => {
                        users.set(user_list);
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    let on_edit_click = {
        let show_update_modal = show_update_modal.clone();
        let sid_input = sid_input.clone();
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let role = role.clone();
        let hours = hours.clone();
        let attendance = attendance.clone();
        let message = message.clone();

        Callback::from(move |user_id: String| {
            let show_update_modal = show_update_modal.clone();
            let sid_input = sid_input.clone();
            let email = email.clone();
            let full_name = full_name.clone();
            let identification = identification.clone();
            let role = role.clone();
            let hours = hours.clone();
            let attendance = attendance.clone();
            let message = message.clone();

            spawn_local(async move {
                match ApiService::admin_get_user(&user_id).await {
                    Ok(user) => {
                        sid_input.set(user.id.clone());
                        email.set(user.email);
                        full_name.set(user.full_name);
                        identification.set(user.identification);

                        match user.role {
                            UserRole::Simple(role_str) => {
                                role.set(role_str);
                                hours.set(0);
                            }
                            UserRole::Speaker { speaker } => {
                                role.set("speaker".to_string());
                                hours.set(speaker.hours);
                            }
                        }

                        attendance.set(user.attendance);
                        show_update_modal.set(true);
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    let on_cancel_modal = {
        let show_update_modal = show_update_modal.clone();
        Callback::from(move |_: MouseEvent| {
            show_update_modal.set(false);
        })
    };

    let on_update_submit = {
        let sid_input = sid_input.clone();
        let email = email.clone();
        let full_name = full_name.clone();
        let identification = identification.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        let role = role.clone();
        let hours = hours.clone();
        let attendance = attendance.clone();
        let message = message.clone();
        let show_update_modal = show_update_modal.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let sid_val = (*sid_input).clone();
            let email_val = (*email).clone();
            let full_name_val = (*full_name).clone();
            let identification_val = (*identification).clone();
            let password_val = (*password).clone();
            let repeated_password_val = (*repeated_password).clone();
            let role_val = (*role).clone();
            let hours_val = *hours;
            let attendance_val = (*attendance).clone();
            let message = message.clone();
            let show_update_modal = show_update_modal.clone();

            if !validate_password(&password_val) || !validate_password(&repeated_password_val) {
                message.set("Contraseña no valida".to_string());
                return;
            }

            if password_val != repeated_password_val {
                message.set("Contraseñas no coinciden".to_string());
                return;
            }

            let data = AdminUpdateUserRequest {
                id: sid_val,
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
                    Ok(response) => {
                        log::info!("Success: {}", response);
                        message.set("Usuario actualizado exitosamente".to_string());
                        show_update_modal.set(false);
                    }
                    Err(error) => {
                        message.set(error);
                    }
                }
            });
        })
    };

    // Event handlers for form inputs
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

    let on_repeated_password_change = {
        let repeated_password = repeated_password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            repeated_password.set(input.value());
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
            if let Ok(value) = input.value().parse::<u32>() {
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

    // Helper functions
    let get_role_display = |user: &User| -> (String, String) {
        match &user.role {
            UserRole::Simple(role_str) => {
                let display = match role_str.as_str() {
                    "webmaster" => "Administrador",
                    "staff" => "Organizador",
                    "attendee" => "Asistente",
                    _ => "Asistente",
                };
                (display.to_string(), "-".to_string())
            }
            UserRole::Speaker { speaker } => ("Ponente".to_string(), speaker.hours.to_string()),
        }
    };

    let get_attendance_display = |attendance: &str| -> &str {
        match attendance {
            "presential" => "Presencial",
            _ => "Remota",
        }
    };

    let get_cert_display = |generated: bool| -> &str {
        if generated {
            "Sí"
        } else {
            "No"
        }
    };

    html! {
        <>
            <h1>{"Administrar usuarios"}</h1>
            <article>
                <section id="buttons-section">
                    <button id="register-btn" onclick={on_register_click}>{"Registrar"}</button>
                    <button id="reload-btn" onclick={on_reload_click}>{"Actualizar lista"}</button>
                </section>

                <section id="table-section">
                    <table>
                        <thead>
                            <tr>
                                <th rowspan="2">{"SID"}</th>
                                <th rowspan="2">{"E-mail"}</th>
                                <th rowspan="2">{"Nombre"}</th>
                                <th rowspan="2">{"Identificación"}</th>
                                <th rowspan="2">{"Rol"}</th>
                                <th rowspan="2">{"Horas"}</th>
                                <th rowspan="2">{"Asistencia"}</th>
                                <th colspan="2">{"Constancias generadas"}</th>
                                <th rowspan="2">{"Acciones"}</th>
                            </tr>
                            <tr>
                                <th>{"Horizontal"}</th>
                                <th>{"Vertical"}</th>
                            </tr>
                        </thead>
                        <tbody id="user-list">
                            {
                                if users.is_empty() {
                                    html! {
                                        <tr>
                                            <td colspan="10">{"Todavía no hay usuarios"}</td>
                                        </tr>
                                    }
                                } else {
                                    users.iter().map(|user| {
                                        let (role_display, hours_display) = get_role_display(user);
                                        let user_id = user.id.clone();
                                        let edit_callback = {
                                            let on_edit_click = on_edit_click.clone();
                                            let user_id = user_id.clone();
                                            Callback::from(move |_: MouseEvent| {
                                                on_edit_click.emit(user_id.clone());
                                            })
                                        };

                                        html! {
                                            <tr key={user.id.clone()}>
                                                <td>{&user.id}</td>
                                                <td>{&user.email}</td>
                                                <td>{&user.full_name}</td>
                                                <td>{&user.identification}</td>
                                                <td>{role_display}</td>
                                                <td>{hours_display}</td>
                                                <td>{get_attendance_display(&user.attendance)}</td>
                                                <td>{get_cert_display(user.cert_generated.horizontal)}</td>
                                                <td>{get_cert_display(user.cert_generated.vertical)}</td>
                                                <td>
                                                    <button onclick={edit_callback}>{"Editar"}</button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </tbody>
                    </table>
                </section>

                // Update Modal
                if *show_update_modal {
                    <div class="modal">
                        <div class="modal-content">
                            <h2>{"Actualizar Usuario"}</h2>
                            <form onsubmit={on_update_submit}>
                                <div class="form-group">
                                    <label>{"SID:"}</label>
                                    <input type="text" value={(*sid_input).clone()} disabled={true} />
                                </div>
                                <div class="form-group">
                                    <label>{"Email:"}</label>
                                    <input type="email" value={(*email).clone()} onchange={on_email_change} />
                                </div>
                                <div class="form-group">
                                    <label>{"Nombre completo:"}</label>
                                    <input type="text" value={(*full_name).clone()} onchange={on_full_name_change} />
                                </div>
                                <div class="form-group">
                                    <label>{"Identificación:"}</label>
                                    <input type="text" value={(*identification).clone()} onchange={on_identification_change} />
                                </div>
                                <div class="form-group">
                                    <label>{"Contraseña:"}</label>
                                    <input type="password" value={(*password).clone()} onchange={on_password_change} />
                                </div>
                                <div class="form-group">
                                    <label>{"Repetir contraseña:"}</label>
                                    <input type="password" value={(*repeated_password).clone()} onchange={on_repeated_password_change} />
                                </div>
                                <div class="form-group">
                                    <label>{"Rol:"}</label>
                                    <select value={(*role).clone()} onchange={on_role_change}>
                                        <option value="attendee">{"Asistente"}</option>
                                        <option value="speaker">{"Ponente"}</option>
                                        <option value="staff">{"Organizador"}</option>
                                    </select>
                                </div>
                                {
                                    if *role == "speaker" {
                                        html! {
                                            <div class="form-group">
                                                <label>{"Horas:"}</label>
                                                <input type="number" value={hours.to_string()} onchange={on_hours_change} />
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <div class="form-group">
                                    <label>{"Asistencia:"}</label>
                                    <select value={(*attendance).clone()} onchange={on_attendance_change}>
                                        <option value="remote">{"Remota"}</option>
                                        <option value="presential">{"Presencial"}</option>
                                    </select>
                                </div>
                                <div class="modal-buttons">
                                    <button type="submit">{"Actualizar"}</button>
                                    <button type="button" onclick={on_cancel_modal}>{"Cancelar"}</button>
                                </div>
                            </form>
                        </div>
                    </div>
                } else {
                    html! {}
                }

                <div>
                    <span id="message-span">{(*message).clone()}</span>
                </div>
            </article>
        </>
    }
}
