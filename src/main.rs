mod components;
mod routes;
mod services;
mod types;
mod utils;

use routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logger");

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// src/routes.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/registro")]
    Register,
    #[at("/ingreso")]
    Login,
    #[at("/perfil")]
    Profile,
    #[at("/constancias")]
    Certificates,
    #[at("/admin/registro")]
    AdminRegister,
    #[at("/admin/usuarios")]
    AdminUsers,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::Profile => html! { <Profile /> },
        Route::Certificates => html! { <Certificates /> },
        Route::AdminRegister => html! { <AdminRegister /> },
        Route::AdminUsers => html! { <AdminUsers /> },
    }
}

pub mod admin_register;
pub mod admin_users;
pub mod certificates;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;

pub use admin_register::AdminRegister;
pub use admin_users::AdminUsers;
pub use certificates::Certificates;
pub use home::Home;
pub use login::Login;
pub use profile::Profile;
pub use register::Register;
