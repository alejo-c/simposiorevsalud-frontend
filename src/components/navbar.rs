use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::auth::AuthService;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let is_authenticated = use_state(|| AuthService::is_authenticated());

    let on_logout = {
        let navigator = navigator.clone();
        let is_authenticated = is_authenticated.clone();
        Callback::from(move |_: MouseEvent| {
            AuthService::logout();
            is_authenticated.set(false);
            navigator.push(&Route::Home);
        })
    };

    html! {
        <nav class="main-nav">
            <div class="nav-container">
                <div class="nav-brand">
                    <Link<Route> to={Route::Home} classes="brand-link">
                        <span class="brand-text">{"SimposioRevSalud"}</span>
                    </Link<Route>>
                </div>

                <ul class="nav-links">
                    <li><Link<Route> to={Route::Home} classes="nav-link">{"Inicio"}</Link<Route>></li>

                    {if *is_authenticated {
                        html! {
                            <>
                                <li><Link<Route> to={Route::Profile} classes="nav-link">{"Perfil"}</Link<Route>></li>
                                <li><Link<Route> to={Route::Certificates} classes="nav-link">{"Certificados"}</Link<Route>></li>
                                <li><Link<Route> to={Route::AdminPanel} classes="nav-link">{"Admin"}</Link<Route>></li>
                                <li>
                                    <button class="btn btn-sm btn-outline" onclick={on_logout}>
                                        {"Cerrar Sesión"}
                                    </button>
                                </li>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <li><Link<Route> to={Route::Register} classes="nav-link">{"Registro"}</Link<Route>></li>
                                <li>
                                    <Link<Route> to={Route::Login} classes="btn btn-sm">
                                        {"Ingresar"}
                                    </Link<Route>>
                                </li>
                            </>
                        }
                    }}
                </ul>

                <div class="nav-toggle">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>
        </nav>
    }
}

// <nav class="custom-nav">
//     <div class="container">
//         <div class="nav-container">
//             <div class="event-title">
//                 <h3>{"I Simposio Internacional de Revistas Científicas"}</h3>
//             </div>
//             <div class="nav-buttons">
//                 {if *is_authenticated {
//                     html! {
//                         <>
//                             <button class="btn btn-outline" onclick={on_profile_click}>
//                                 {"Perfil"}
//                             </button>
//                             <button class="btn btn-secondary" onclick={on_logout_click}>
//                                 {"Cerrar Sesión"}
//                             </button>
//                         </>
//                     }
//                 } else {
//                     html! {
//                         <>
//                             <button class="btn btn-outline" onclick={on_register_click}>
//                                 {"Registro"}
//                             </button>
//                             <button class="btn" onclick={on_login_click}>
//                                 {"Ingreso"}
//                             </button>
//                         </>
//                     }
//                 }}
//             </div>
//         </div>
//     </div>
// </nav>
