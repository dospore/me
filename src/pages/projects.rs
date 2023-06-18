use yew::prelude::*;

use crate::components::project::{
    ProjectItem,
    Project
};

#[function_component(Projects)]
pub fn projects() -> Html {

    let projects = vec![
        ProjectItem {
            name: "Test".to_string(),
            duration: "Whole life".to_string(),
            description: "This NFT granted me access to an online clubhouse platform, enabling interactions with other holders and active participation in a community focused on the asset. Additionally, I created a Twitter profile that complemented the NFT's aesthetic, fostering connections and engagement with fellow holders. Thank you for your kind attention. I eagerly await your response.".to_string()
        }
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
