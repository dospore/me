use yew::prelude::*;

// use crate::components::animated_image;

use crate::components::animated_image::AnimatedImage;

#[function_component(Collection)]
pub fn collection() -> Html {
    html! {
        <div class="collection">
            <AnimatedImage collection="broker" />
            <AnimatedImage collection="punk" />
            <AnimatedImage collection="captain" />
        </div>
    }
}
