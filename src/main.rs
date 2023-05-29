use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::Route;

pub mod utils;

mod pages;
use pages::{
    collection::Collection,
    projects::Projects,
    home::Home,
};

pub mod components;
use components::{
    header::Header,
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
    html! {
        <HashRouter>
            <Header />
            <Switch <Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
