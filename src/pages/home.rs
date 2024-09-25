use yew::prelude::*;
use web_sys::window;

use crate::components::{
    color_box::ColorBox,
    modal_box::ModalBox,
    popup_info::PopupInfo
};

#[function_component(Home)]
pub fn home() -> Html {
    let popups: UseStateHandle<[bool; 11]> = use_state(|| [false; 11]);
    let popup_count = use_state(|| 0);

    let inc_count = {
        let popup_count = popup_count.clone();
        let popups = popups.clone();

        Callback::from(move |_| {
            let mut new_array = *popups;
            new_array[*popup_count] = true;
            popups.set(new_array);
            popup_count.set(*popup_count + 1);
        })
    };

    let handle_close = {
        let popups = popups.clone();
        Callback::from(move |index: usize| {
            let mut new_array = *popups;
            new_array[index] = false;
            popups.set(new_array);
        })
    };


    let invert = {
        Callback::from(move |_| {
            let window = window().expect("should have a window in this context");
            let html = window.document().unwrap().body().unwrap().parent_element().unwrap();
            html.set_class_name(
                match html.class_name().as_str() {
                    "invert" => "",
                    _ => "invert"
                }
            )
        })
    };

    html! {
        <>
            <ModalBox>
                <div class="content" style="position: relative;">
                    <h1 class="dospore">
                        {"dospore"}
                    </h1>
                    <div class="row">
                        {"Mostly dos a little bit of mushroom"}
                    </div>
                    <div class="row">
                        {"Theme: "}
                        <div class={"colors"}>
                            <ColorBox color="var(--bg)" name="bg" />
                            <ColorBox color="var(--text)" name="text" />
                            <ColorBox color="var(--accent1)" name="accent1" />
                            <ColorBox color="var(--accent2)" name="accent2" />
                        </div>
                    </div>
                    <div class="row">
                        {"Github: "}<a target="_blank" href={"https://github.com/dospore"}>{"goto github.dospore"}</a>
                    </div>
                    <div class="row">
                        {"Twitter: "}<a target="_blank" href={"https://twitter.com/_dospore"}>{"goto twitter._dospore"}</a>
                    </div>
                    <div class="row">
                        {"Exercism: "}<a target="_blank" href={"https://exercism.org/profiles/dospore"}>{"goto exercism.Dospore"}</a>
                    </div>
                    <div class="row">
                        {"Discord: "}<a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"goto discord.Dospore#6811"}</a>
                    </div>
                    <br />
                    <button class="close" onclick={inc_count} disabled={*popup_count >= 11}>
                        {"Give me more"}
                    </button>
                    <div class="row">
                        {"Contact me on "} 
                        <a target="_blank" href={"https://twitter.com/_dospore"}>{"twitter"}</a>
                        {" or "}
                        <a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"discord"}</a>
                    </div>
                    <button class="vintage" style="font-size: 12px; opacity: 0.8; border-color: white; position: absolute; right: 0; bottom: 0;" onclick={invert}>
                        {"Try me"}
                        <img style="padding-left: 4px; width: 16px; height: auto; display: inline-block;" src="static/reverse.svg" alt="reverse" />
                    </button>
                </div>
            </ModalBox>
            <PopupInfo popups={*popups} handle_close={handle_close.clone()} />
        </>
    }
}
