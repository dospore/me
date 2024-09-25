use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct BoxProps{
    // #[prop_or_default]
    pub children: Children,
}

#[function_component(ModalBox)]
pub fn modal_box(props: &BoxProps) -> Html {
    html! {
        <div class={classes!("modal-box")}>
            {props.children.clone()}
        </div>
    }

}

