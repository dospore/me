use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
        html! {
            <main>
                <div class="header">
                    <div class="letters">
                        <span class="d">{"d"}</span>
                        <span class="o">{"ospore"}</span>
                        <span class="pronounce">{"(dos-spore)"}</span>
                    </div>
                    <div class="nav">
                        // <span class="link">
                            // <Tooltip>
                                // <div class="box">
                                    // {"Hover me please"}
                                // </div>
                            // </Tooltip>
                        // </span>
                    </div>
                </div>
                <div>
                    {"Mostly dos a little bit of mushroom"}
                </div>
                // <Lines start={250} />
            </main>
        }
}
