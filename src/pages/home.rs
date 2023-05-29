use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
        html! {
            <div>
                <h1 class="dospore">
                    {"dospore"}
                </h1>
                <div>
                    {"Mostly dos a little bit of mushroom"}
                </div>
            </div>
        }
}
