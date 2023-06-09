use yew::prelude::*;

use crate::components::tooltip::Tooltip;

#[derive(PartialEq, Properties, Clone)]
pub struct ColorBoxProps {
    pub color: String,
    pub name: String
}
#[function_component(ColorBox)]
pub fn color_box(props: &ColorBoxProps) -> Html {

    let border: String = match props.name.as_str() {
        "bg" => String::from("var(--text)"),
        _ => props.color.clone()
    };

    html! {
        <Tooltip label={props.name.clone()}>
            <span style={format!("background: {}; border: 1px solid {};", props.color.clone(), border)} class={"color-box"} />
        </Tooltip>
    }
}
