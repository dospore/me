use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="nav">
                        <span class="link">
                            <a href="/">{"cd"}</a>
                        </span>
                        <span class="seperator">{"|"}</span>
                        <span class="link">
                            <a href="/projects">{"cd /projects"}</a>
                        </span>
                        <span class="seperator">{"|"}</span>
                        <span class="link">
                            <a href="/collection">{"cd /collection"}</a>
                        </span>
        </div>
    }

}
