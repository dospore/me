use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div>
                        <span class="link">
                            <a href="/projects">{"/projects"}</a>
                        </span>
                        <span class="seperator">{"|"}</span>
                        <span class="link">
                            <a href="/collection">{"/collection"}</a>
                        </span>
        </div>
    }

}
