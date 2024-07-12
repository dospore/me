use yew::prelude::*;

#[derive(PartialEq)]
pub struct ProjectItem {
    pub name: &'static str,
    pub duration: &'static str,
    pub role: &'static str,
    pub link: &'static str,
    pub description: &'static str 
}

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    // #[prop_or_default]
    // pub children: Children,
    pub project: ProjectItem,
}


#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    html! {
        <div class="project">
            <h2 class="project-title"><a target="_blank" href={props.project.link}>{&props.project.name}</a></h2>
            <p class="project-duration">{"Duration: "}{&props.project.duration}</p>
            <p class="project-role">{"Role: "}{&props.project.role}</p>
            <p class="project-description">{&props.project.description}</p>
        </div>
    }
}
