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
                    <button class={classes!("vintage")}>
                    <span class="dim">{"cd"}</span>
                    </button>
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
                    <button class="vintage">
                        <span class="dim">{"cd "}</span>
                        {"/projects"}
                    </button>
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
                    <button class="vintage">
                        <span class="dim">{"cd "}</span>
                        {"/collection"}
                    </button>
                </Link<Route>>
            </span>
        </div>
    }

}
