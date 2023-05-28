use yew::prelude::*;
use yew_router::prelude::*;

pub mod utils;

mod pages;
use pages::{
    collection::Collection,
    home::Home,
};

pub mod components;
use components::{
    header::Header,
};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/collection")]
    Collection,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Collection => html! {
            <Collection />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch <Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
