use yew::prelude::*;
use rand::Rng;

use crate::utils::{get_screen_type,ScreenType};

use crate::components::popup::Popup;

#[derive(Properties, PartialEq)]
pub struct PopupInfoProps{
    // #[prop_or_default]
    pub popups: [bool; 11],
    pub handle_close: Callback<usize>
}

pub fn get_random_coords(screen_type: ScreenType) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();

    match screen_type {
        ScreenType::Desktop => {
            (0..10)
                .map(|_| {
                    let x = rng.gen_range(90..910);  // Random x values between 90 and 910
                    let y = rng.gen_range(140..660); // Random y values between 140 and 660
                    (x, y)
                })
                .chain(std::iter::once((400, 400))) // Add the additional point
                .collect()
        }
        ScreenType::Mobile => {
            (0..10)
                .map(|_| {
                    let x = rng.gen_range(4..50);  // Random x values between 40 and 260
                    let y = rng.gen_range(90..610);  // Random y values between 90 and 610
                    (x, y)
                })
                .chain(std::iter::once((20, 280))) // Add the additional point
                .collect()
        }
    }
}

#[function_component(PopupInfo)]
pub fn popup_info(props: &PopupInfoProps) -> Html {
    let content: Vec<Html> = vec![
        html! { <div class="row">{"Dospore is a full-stack developer specializing in dapp development."}</div> },
        html! { <div class="row">{"Dospore is passionate about data security, efficient systems, best practices, and open-source development."}</div> },
        html! { <div class="row">{"Dospore does not like python."}</div> },
        html! { <div class="row">{"Dospore mostly builds with React, Node, and Foundry but is currently improving his Rust -- that is why this website is written in Rust :)"}</div> },
        html! { <div class="row">{"Dospore likes C and Rust."}</div> },
        html! { <div class="row">{"Dospore has had as much fun learning and writing Rust as they did when they originally learned C in university."}</div> },
        html! { <div class="row">{"Dospore's thesis on "}<a target="_blank" href="https://github.com/dospore/dospore/blob/29b37db4dafb9b888538507f4bdf99d32fbe2a6a/static/Automatically_Patching_Security_Vulnerabilities_in_Software_Using_Patcherex.pdf">{"Automatic Patching of Security Vulnerabilities in Software during runtime"}</a></div> },
        html! { <div class="row">{"Originally studied Biomedical Electrical Engineering but preferred writing as much code as possible. "}{"Biology and the intersection of biology and physics continues to be a passion of Dospore's, and he continues to read/learn more about it."}</div> },
        html! { <div class="row">{"I (!Dospore) graduated Software Engineering from the University of Queensland in 2020 and moved straight into a web3 startup."}</div> },
        html! { <div class="row">{"Dospore and I are seeking deep knowledge, best coding practices, and more from experienced engineers after figuring a lot of sh*t amongst our small team of web3 tech grads."}</div> },
        html! {
            <div class="row">
                {"Just go ahead and ask me out already..."}
                <div> 
                    {"Contact me on "} 
                    <a target="_blank" href={"https://twitter.com/_dospore"}>{"twitter"}</a>
                    {" or "}
                    <a target="_blank" href={"https://discord.com/users/810701025353924669"}>{"discord"}</a>
                </div>
            </div>
        },
    ];

    let coordinates = use_state(|| get_random_coords(get_screen_type()));

    html! {
        <div>
        { for props.popups.into_iter().enumerate()
            .map(
                |(index, display)| {
                    if !display {
                        html!{}
                    } else {
                        let (x_offset, y_offset) = coordinates[index];
                        let handle_close = props.handle_close.clone();
                        let onclose = {
                            // Pass the current index to the callback
                            Callback::from(move |_| {
                                handle_close.emit(index); // Emit the index when the button is clicked
                            })
                        };

                        let info = &content[index];

                        html!{
                            <Popup x_offset={x_offset} y_offset={y_offset}>
                                {info.clone()}
                                <button class="close" style="border-bottom-right-radius: 0px; position: absolute; bottom: 0px; right: 0px;" onclick={onclose}>
                                    {"X"}
                                </button>
                            </Popup>
                        }
                    }
                }
            )
        }
        </div>
    }

}

