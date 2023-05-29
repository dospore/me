use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::Route;

#[function_component(Header)]
pub fn header() -> Html {

    let route: Route = use_route().unwrap();

    html! {
        <div class={classes!("nav", "code")}>
            <span class={
                classes!(
                    "link",
                    if route == Route::Home { "selected" } else { "" }
                )}
            >
                <Link <Route> to={Route::Home}>
                    <span class="dim">{"cd"}</span>
                </Link<Route>>
            </span>
            <span class="seperator">{"|"}</span>
            <span class={
                classes!(
                    "link",
                    if route == Route::Projects { "selected" } else { "" }
                )}
            >
                <Link <Route> to={Route::Projects}>
                    <span class="dim">{"cd "}</span>
                    {"/projects"}
                </Link<Route>>
            </span>
            <span class="seperator">{"|"}</span>
            <span class={
                classes!(
                    "link",
                    if route == Route::Collection { "selected" } else { "" }
                )}
            >
                <Link <Route> to={Route::Collection}>
                    <span class="dim">{"cd "}</span>
                    {"/collection"}
                </Link<Route>>
            </span>
        </div>
    }

}
