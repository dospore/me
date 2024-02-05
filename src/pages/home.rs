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
                <div class="row">
                    {"Exercism: "}<a target="_blank" href={"https://exercism.org/profiles/dospore"}>{"goto exercism.Dospore"}</a>
                </div>
                <div class="row">
                    {"Discord: "}<a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"goto discord.Dospore#6811"}</a>
                </div>
                <br />
                <div class="row">
                    {"Dospore is a full-stack developer specializing in dapp development. "}
                </div>
                <div class="row">
                    {"Dospore does not like python. "}
                </div>
                <div class="row">
                    {"Dospore mostly build with React, node and hardhat but am currently improving my Rust -- that is why this website is written in rust :)"}
                </div>
                <div class="row">
                    {"Dospore is passionate about data security, efficient systems, best practices and opensource development. "}
                </div>
                <div class="row">
                    {"Dospore does like c. "}
                </div>
                <div class="row">
                    {"Dospore has had as much fun learning and writing Rust as they did when they originally learned c in University. "}
                </div>
                <div class="row">
                    {"Dospore's thesis on "}<a target="_blank" href="https://github.com/dospore/dospore/blob/29b37db4dafb9b888538507f4bdf99d32fbe2a6a/static/Automatically_Patching_Security_Vulnerabilities_in_Software_Using_Patcherex.pdf">{"Automatic Patching of Security Vulnerabilities in Software during runtime"}</a>
                </div>
                <div class="row">
                    {"Originally studied Biomedical Electrical Engineering but preferred writing as much code as possible. "}
                    {"Biology and the intersection of biology and physics continues to be passion of Dospore's and he continues to read/learn more about it. "}
                </div>
                <div class="row">
                    {"I (!Dospore) Graduated Software Engineering from the University of Queensland in 2020 and moved straight into a web3 startup. "}
                </div>
                <div class="row">
                    {"Dospore and I are seeking deep knowledge, best coding practices and more from experienced engineers after figuring a lot of sh*t amongst our small team of web3 tech grads. "}
                </div>
                <div class="row">
                    {"Contact me on "} 
                    <a target="_blank" href={"https://twitter.com/_dospore"}>{"twitter"}</a>
                    {" or "}
                    <a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"discord"}</a>
                </div>
            </div>
        }
}
