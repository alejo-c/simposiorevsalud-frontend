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
