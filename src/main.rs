use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::use_state;

use crate::utils::Route;

pub mod utils;
pub mod helpers;

mod pages;
use pages::{
    collection::Collection,
    projects::Projects,
    home::Home,
};

pub mod components;
use components::{
    header::Header,
    lines::Lines
};


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Collection => html! {
            <Collection />
        },
        Route::Projects => html! {
            <Projects />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


#[function_component(App)]
fn app() -> Html {
    let enable_weirdness = use_state(|| false);

    let onclick = {
        let weirdness = enable_weirdness.clone();
        Callback::from(move |_| weirdness.set(!(*weirdness)))
    };

    html! {
        <HashRouter>
            <Header />
            <Switch <Route> render={switch} />
            <div class="content">
                if *enable_weirdness{ <Lines /> }
                <button class="lets-get-weird" onclick={onclick}>
                    if *enable_weirdness { <>{"Stop"}</> } else { <>{"Start"}</> }
                    {" weirdness"}
                </button>
            </div>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
