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
    #[at("/admin/usuarios")]
    AdminUsers,
    #[at("/admin/registro")]
    AdminRegister,
    #[at("/admin/usuario/:id")]
    AdminUpdate { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::Profile => html! { <Profile /> },
        Route::Certificates => html! { <Certificates /> },
        Route::AdminUsers => html! { <AdminUsers /> },
        Route::AdminRegister => html! { <AdminRegister /> },
        Route::AdminUpdate { id } => html! { <AdminUpdate user_id={id} /> },
        Route::NotFound => html! { <NotFound />},
    }
}
