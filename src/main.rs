use yew::prelude::*;
use tooltip::Tooltip;
use lines:: Lines;

mod tooltip;
mod lines;
pub mod utils;

pub enum Msg {
}

pub struct App {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <main>
                <div class="header">
                    <div class="letters">
                        <span class="d">{"d"}</span>
                        <span class="o">{"ospore"}</span>
                        <span class="pronounce">{"(dos-spore)"}</span>
                    </div>
                    // <div class="nav">
                        // <span class="link">
                            // <Tooltip>
                                // <div class="box">
                                    // {"Hover me please"}
                                // </div>
                            // </Tooltip>
                        // </span>
                        // <span class="seperator">{"|"}</span>
                        // <span class="link">
                            // <a href="/projects">{"/projects"}</a>
                        // </span>
                    // </div>
                </div>
                <div>
                    {"Mostly dos a little bit of mushroom"}
                </div>
                <Lines start={200} />
            </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
