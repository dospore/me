use yew::prelude::*;

#[derive(PartialEq)]
pub struct ProjectItem {
    pub name: String,
    pub duration: String,
    pub description: String
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
        <div>
            <h2>{&props.project.name}</h2>
            <p>{"Duration: "}{&props.project.duration}</p>
            <p>{&props.project.description}</p>
        </div>
    }
}
