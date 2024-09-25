use yew::prelude::*;
use crate::components::{
    modal_box::ModalBox,
    animated_image::AnimatedImage
};

#[function_component(Collection)]
pub fn collection() -> Html {
    html! {
        <ModalBox>
            <div class="collection">
                <h1>{"The Vault"}</h1>
                <p>
                    {"If I had to make a justification on my gambling addictions I'd say I like jpegs which in some way are technologically innovative. "}
                    {"I bought many"}
                    <span class="redacted">
                        <span>{" ["}</span>
                        <span>{"redacted"}</span>
                        <span>{"] "}</span>
                    </span>
                    {"jpegs which are not listed here."}
                </p>
                <p>
                    {"// This collection is screenshotted and has no real value"}
                    <br />
                    {"// TODO make not screenshottable"}
                </p>
                <div class="assets">
                    <AnimatedImage collection="broker" />
                    // <AnimatedImage collection="punk" />
                    // <AnimatedImage collection="captain" />
                </div>
            </div>
        </ModalBox>
    }
}
