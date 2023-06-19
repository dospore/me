use yew::prelude::*;

use crate::components::project::{
    ProjectItem,
    Project
};

#[function_component(Projects)]
pub fn projects() -> Html {

    let projects = vec![
        ProjectItem {
            name: "Mycelium.xyz".to_string(),
            link: "https://mycelium.xyz/".to_string(),
            role: "Full-stack Develeoper".to_string(),
            duration: "Long time".to_string(),
            description: "Full-stack developer at Mycelium. Core team building decentralised Perpetual Swaps and Perpetual Pool mechanisms.".to_string()
        },
        ProjectItem {
            name: "This Website".to_string(),
            link: "https://github.com/dospore/me".to_string(),
            role: "Rustacean in training".to_string(),
            duration: "Indefinite".to_string(),
            description: "I am trying to improve my Rust skills through something I know well (web development). If you are asking yourself 'WTF is this' the answer is probably me attempting to showcase the power of wasm.".to_string()
        },
    ];

    html! {
        <div class="projects">
            <h1>{"Projects"}</h1>
            <p class="code">
                {"// TODO do more stuff / list more projects"}
            </p>
            {for projects.into_iter().map(|p| html! { <Project project={p} /> })}
        </div>
    }
}
