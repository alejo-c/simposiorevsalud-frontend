use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::services::auth::AuthService;

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps {
    pub children: Children,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &ProtectedRouteProps) -> Html {
    let navigator = use_navigator().unwrap();
    let is_authenticated = use_state(|| AuthService::is_authenticated());
    let id_authenticated_copy = *is_authenticated.clone();

    use_effect_with_deps(
        move |_| {
            if !id_authenticated_copy {
                navigator.push(&Route::Login);
            }
            || ()
        },
        id_authenticated_copy.clone(),
    );

    if *is_authenticated {
        html! {
            <>
                { for props.children.iter() }
            </>
        }
    } else {
        html! {
            <div class="container">
                <div class="spinner"></div>
                <p>{"Verificando autenticación..."}</p>
            </div>
        }
    }
}
