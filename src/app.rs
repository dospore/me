use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div>
                <div class="letters">
                    <span class="d">{ "d" }</span>
                    <span class="o">{ "o" }</span>
                    <span class="s">{ "s" }</span>
                    <span class="p">{ "p" }</span>
                    <span class="o2">{ "o" }</span>
                    <span class="r">{ "r" }</span>
                    <span class="e">{ "e" }</span>
                    <span class="pronounce">{"(dos-spore)"}</span>
                </div>
            </div>
            <div>
                {"Mostly dos a little bit of mushroom"}
            </div>
        </main>
    }
}
