use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::types::{User, UserRole, AdminUpdateUserRequest, DeleteUserRequest};
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
            UserRole::Speaker { speaker } => {
                ("Ponente".to_string(), speaker.hours.to_string())
            }
        }
    };

    let get_attendance_display = |attendance: &str| -> &str {
        match attendance {
            "presential" => "Presencial",
            _ => "Remota",
        }
    };

    let get_cert_display = |generated: bool| -> &str {
        if generated { "Sí" } else { "No" }
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
                                        let user_id =
