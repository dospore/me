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
            description: "I am trying to improve my Rust skills through something I know well (web development). If you are asking yourself 'WTF is this' I don't have a good answer".to_string()
        },
        ProjectItem {
            name: "Particles.rs".to_string(),
            link: "https://particles-rs.vercel.app/".to_string(),
            role: "Rustacean in training".to_string(),
            duration: "Rainy weekend".to_string(),
            description: "Ported the popular particles.js library to wasm/rust to experiment with wasm. This was not because particles.js is not performant, more of a learning excercise. Canvas interactions through rs are relatively slow compared to native js so I followed the approach recommended by \"the people of the internet\". Store state in rust and read directly from wasm memory to draw to the canvas.".to_string()
        },
        ProjectItem {
            name: "Memeland Traits Wardrobe".to_string(),
            link: "https://memeland-traits-canvas.vercel.app/".to_string(),
            role: "Discord Sh*ttalker".to_string(),
            duration: "Rainy weekend".to_string(),
            description: "Inclusive Memeland".to_string()
        },
        ProjectItem {
            name: "Aavegotchi Battler Stats".to_string(),
            link: "https://dospore.github.io/gotchi-stats-calculator/".to_string(),
            role: "Rustacean in training".to_string(),
            duration: "Rainy weekend".to_string(),
            description: "I wanted to build an AAVEgotchi Battler team so I put together a stats calculator using yew.rs to help me visualise the stats of the gotchi's I would be purchasing. I'll take any chance I can get to practice some Rust".to_string()
        },
    ];

    html! {
        <div class="projects">
            <h1>{"Projects"}</h1>
            <p>{"// TODO do more stuff / list more projects"}</p>
            {for projects.into_iter().map(|p| html! { <Project project={p} /> })}
        </div>
    }
}
