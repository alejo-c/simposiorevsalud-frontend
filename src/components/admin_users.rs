use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::api::ApiService;
use crate::types::{User, UserRole};

#[function_component(AdminUsers)]
pub fn admin_users() -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let users = use_state(|| Vec::<User>::new());
    let message = use_state(|| String::new());

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
                                                    <button>{"Editar"}</button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </tbody>
                    </table>
                </section>

                <div>
                    <span id="message-span">{(*message).clone()}</span>
                </div>
            </article>
        </>
    }
}
