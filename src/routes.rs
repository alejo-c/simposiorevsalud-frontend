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
        // Public routes
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },

        // Protected routes
        Route::Profile => html! {
            <ProtectedRoute>
                <Profile />
            </ProtectedRoute>
        },
        Route::Certificates => html! {
            <ProtectedRoute>
                <Certificates />
            </ProtectedRoute>
        },
        Route::AdminUsers => html! {
            <ProtectedRoute>
                <AdminUsers />
            </ProtectedRoute>
        },
        Route::AdminRegister => html! {
            <ProtectedRoute>
                <AdminRegister />
            </ProtectedRoute>
        },
        Route::AdminUpdate { id } => html! {
            <ProtectedRoute>
                <AdminUpdate user_id={id} />
            </ProtectedRoute>
        },
    }
}
