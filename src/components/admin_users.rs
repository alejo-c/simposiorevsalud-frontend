use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::types::{DeleteUserRequest, User, UserRole};

#[function_component(AdminUsers)]
pub fn admin_users() -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let users = use_state(|| Vec::<User>::new());
    let message = use_state(|| String::new());
    let deleting_user = use_state(|| None::<String>);

    // Load users on component mount
    {
        let users = users.clone();
        let message = message.clone();
        use_effect_with_deps(
            move |_| {
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
            },
            (),
        );
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

    let on_delete_click = {
        let users = users.clone();
        let message = message.clone();
        let deleting_user = deleting_user.clone();

        Callback::from(move |user: User| {
            let users = users.clone();
            let message = message.clone();
            let deleting_user = deleting_user.clone();

            // Confirm deletion
            if gloo_utils::window()
                .confirm_with_message(&format!(
                    "¿Está seguro que desea eliminar al usuario {}?",
                    user.full_name
                ))
                .unwrap_or(false)
            {
                deleting_user.set(Some(user.id.clone()));

                let delete_request = DeleteUserRequest {
                    email: user.email,
                    full_name: user.full_name,
                    identification: user.identification,
                    password: String::new(), // This might need to be handled differently
                    role: match user.role {
                        UserRole::Simple(ref role) => role.clone(),
                        UserRole::Speaker { .. } => "speaker".to_string(),
                    },
                    hours: user.hours.unwrap_or(0),
                    attendance: user.attendance,
                };

                spawn_local(async move {
                    match ApiService::delete_user(delete_request).await {
                        Ok(_) => {
                            // Reload users after successful deletion
                            match ApiService::admin_get_users().await {
                                Ok(user_list) => {
                                    users.set(user_list);
                                    message.set("Usuario eliminado exitosamente".to_string());
                                }
                                Err(error) => {
                                    message.set(error);
                                }
                            }
                        }
                        Err(error) => {
                            message.set(format!("Error al eliminar usuario: {}", error));
                        }
                    }
                    deleting_user.set(None);
                });
            }
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
        <div class="container">
            <h1>{"Administrar usuarios"}</h1>
            <article>
                <section id="buttons-section" class="admin-actions">
                    <button id="register-btn" class="btn" onclick={on_register_click}>{"Registrar"}</button>
                    <button id="reload-btn" class="btn btn-outline" onclick={on_reload_click}>{"Actualizar lista"}</button>
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
                                        let user_clone = user.clone();
                                        let on_delete = on_delete_click.clone();
                                        let is_deleting = deleting_user.as_ref() == Some(&user.id);

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
                                                    <div class="admin-actions" style="gap: 0.5rem;">
                                                        <Link<Route> to={Route::AdminUpdate { id: user.id.clone() }}>
                                                            <button class="btn btn-sm">{"Editar"}</button>
                                                        </Link<Route>>
                                                        <button
                                                            class="btn btn-danger btn-sm"
                                                            onclick={move |_| on_delete.emit(user_clone.clone())}
                                                            disabled={is_deleting}
                                                        >
                                                            {if is_deleting {
                                                                html! { <span class="spinner" style="width: 1rem; height: 1rem;"></span> }
                                                            } else {
                                                                html! { {"Eliminar"} }
                                                            }}
                                                        </button>
                                                    </div>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </tbody>
                    </table>
                </section>

                {if !message.is_empty() {
                    html! {
                        <div>
                            <span id="message-span">{(*message).clone()}</span>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </article>
        </div>
    }
}
