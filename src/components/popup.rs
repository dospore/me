use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps{
    // #[prop_or_default]
    pub children: Children,
    pub x_offset: u32,
    pub y_offset: u32,
}

#[function_component(Popup)]
pub fn popup(props: &PopupProps) -> Html {

    html! {
        <div class={classes!("popup")} style={format!("left: {}px; top: {}px", props.x_offset, props.y_offset)}>
            {props.children.clone()}
        </div>
    }

}

