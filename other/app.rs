use crate::components::{AdminPanel, Dashboard, Header, Login, Register};
use crate::services::AuthService;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Route {
    Home,
    Register,
    Login,
    Profile,
    AdminPanel,
    AdminRegister,
    AdminUpdate,
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    let current_route = use_state(|| Route::Home);
    let user_state = use_state(|| None::<crate::services::NetlifyUser>);

    use_effect_with((), |_| {
        AuthService::init();

        // Check if user is already logged in
        if let Some(user) = AuthService::get_current_user() {
            user_state.set(Some(user));
        }

        || {}
    });

    let on_route_change = {
        let current_route = current_route.clone();
        Callback::from(move |route: Route| {
            current_route.set(route);
        })
    };

    let on_register = {
        let current_route = current_route.clone();
        Callback::from(move |user: crate::services::NetlifyUser| {
            current_route.set(Route::Login);
        })
    };

    let on_login = {
        let user_state = user_state.clone();
        let current_route = current_route.clone();
        Callback::from(move |user: crate::services::NetlifyUser| {
            user_state.set(Some(user));
            current_route.set(Route::Profile);
        })
    };

    let on_logout = {
        let user_state = user_state.clone();
        let current_route = current_route.clone();
        Callback::from(move |_| {
            AuthService::logout_user();
            user_state.set(None);
            current_route.set(Route::Home);
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50">
            <Header
                current_route={(*current_route).clone()}
                user={(*user_state).clone()}
                on_route_change={on_route_change.clone()}
                on_logout={on_logout}
            />

            <main class="container mx-auto px-4 py-8">
                {match (*current_route).clone() {
                    Route::Home => html! {
                        <div class="text-center">
                            <h1 class="text-4xl font-bold text-gray-900 mb-8">
                                {"Bienvenido al Simposio RevSalud"}
                            </h1>
                            <p class="text-xl text-gray-600 mb-8">
                                {"Registro y gestión de certificados para el evento"}
                            </p>

                            {if user_state.is_none() {
                                html! {
                                    <div class="space-x-4">
                                        <button
                                            class="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700"
                                            onclick={
                                                let on_route_change = on_route_change.clone();
                                                Callback::from(move |_| on_route_change.emit(Route::Register))
                                            }
                                        >
                                            {"Registrarse"}
                                        </button>
                                        <button
                                            class="bg-gray-600 text-white px-6 py-3 rounded-lg hover:bg-gray-700"
                                            onclick={Callback::from(move |_| AuthService::open_auth())}
                                        >
                                            {"Iniciar Sesión"}
                                        </button>
                                    </div>
                                }
                            } else {
                                html! {
                                    <button
                                        class="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700"
                                        onclick={
                                            let on_route_change = on_route_change.clone();
                                            Callback::from(move |_| on_route_change.emit(Route::Dashboard))
                                        }
                                    >
                                        {"Ver Dashboard"}
                                    </button>
                                }
                            }}
                        </div>
                    },
                    Route::Register => html! {
                        <Register on_login={on_login.clone()} on_route_change={on_route_change.clone()} />
                    },
                    Route::Login => html! {
                        <div class="text-center">
                            <h2 class="text-2xl font-bold mb-4">{"Iniciar Sesión"}</h2>
                            <button
                                class="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700"
                                onclick={Callback::from(move |_| AuthService::open_auth())}
                            >
                                {"Abrir Login con Netlify Identity"}
                            </button>
                        </div>
                    },
                    Route::Dashboard => {
                        if let Some(user) = (*user_state).clone() {
                            html! { <Dashboard user={user} /> }
                        } else {
                            html! {
                                <div class="text-center">
                                    <p>{"Debes iniciar sesión para acceder al dashboard."}</p>
                                    <button
                                        class="bg-blue-600 text-white px-4 py-2 rounded mt-4"
                                        onclick={Callback::from(move |_| AuthService::open_auth())}
                                    >
                                        {"Iniciar Sesión"}
                                    </button>
                                </div>
                            }
                        }
                    },
                    Route::Admin => {
                        if AuthService::is_admin() {
                            html! { <AdminPanel /> }
                        } else {
                            html! {
                                <div class="text-center">
                                    <p>{"No tienes permisos para acceder a esta sección."}</p>
                                </div>
                            }
                        }
                    }
                }}
            </main>
        </div>
    }
}
