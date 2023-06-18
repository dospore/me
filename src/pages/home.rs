use yew::prelude::*;

use crate::components::{
    color_box::ColorBox
};

#[function_component(Home)]
pub fn home() -> Html {
        html! {
            <div class="content">
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
                <div>
                    {"Discord: "}<a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"goto discord.Dospore#6811"}</a>
                </div>
                <br />
                <div class="row">
                    {"I'm a full-stack developer specializing in dapp development. "}
                    {"I am passionate about data security, inneficcient systems/practices and opensource development. "}
                    {"I mostly build with React, node and hardhat but am currently improving my Rust -- that is why this website is written in rust :)"}
                    <br />
                    <br />
                    {"Contact me on "} 
                    <a target="_blank" href={"https://twitter.com/_dospore"}>{"twitter"}</a>
                    {" or "}
                    <a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"discord"}</a>
                </div>
            </div>
        }
}
