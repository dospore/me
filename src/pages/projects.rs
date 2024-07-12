use yew::prelude::*;

use crate::components::project::{
    ProjectItem,
    Project
};

#[function_component(Projects)]
pub fn projects() -> Html {

    let projects = vec![
        ProjectItem {
            name: "Peak Studios",
            link: "https://peak-studios.vercel.app/",
            role: "Drone Pilot",
            duration: "Hobby",
            description: "FPV drone services inspired by using FPV drones for bridge inspections."
        },
        ProjectItem {
            name: "Pendle (P)Lottery",
            link: "https://pendle-pottery-ui.vercel.app/",
            role: "Full-stack Develeoper",
            duration: "2 days 3 long nights",
            description: "Built a lottery dapp during ETH Sydney (an ETH Global) hackathon with two smart contract engineers. Won the best defi app deployed to mantl (prize of 3k USD)"
        },
        ProjectItem {
            name: "Mycelium.xyz",
            link: "https://mycelium.xyz/",
            role: "Full-stack Develeoper",
            duration: "Long time",
            description: "Full-stack developer at Mycelium. Core team building decentralised Perpetual Swaps and Perpetual Pool mechanisms."
        },
        ProjectItem {
            name: "This Website",
            link: "https://github.com/dospore/me",
            role: "Rustacean in training",
            duration: "Indefinite",
            description: "I am trying to improve my Rust skills through something I know well (web development). If you are asking yourself 'WTF is this' I don't have a good answer"
        },
        ProjectItem {
            name: "Particles.rs",
            link: "https://particles-rs.vercel.app/",
            role: "Rustacean in training",
            duration: "Rainy weekend",
            description: "Ported the popular particles.js library to wasm/rust to experiment with wasm. This was not because particles.js is not performant, more of a learning excercise. Canvas interactions through rs are relatively slow compared to native js so I followed the approach recommended by \"the people of the internet\". Store state in rust and read directly from wasm memory to draw to the canvas."
        },
        ProjectItem {
            name: "Memeland Traits Wardrobe",
            link: "https://memeland-traits-canvas.vercel.app/",
            role: "Discord Sh*ttalker",
            duration: "Rainy weekend",
            description: "Inclusive Memeland"
        },
        ProjectItem {
            name: "Aavegotchi Battler Stats",
            link: "https://dospore.github.io/gotchi-stats-calculator/",
            role: "Rustacean in training",
            duration: "Rainy weekend",
            description: "I wanted to build an AAVEgotchi Battler team so I put together a stats calculator using yew.rs to help me visualise the stats of the gotchi's I would be purchasing. I'll take any chance I can get to practice some Rust"
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
