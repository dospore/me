use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug)]
pub struct Tooltip {
    show_tooltip: bool,
}

impl Tooltip {
    fn show(&mut self) {
        self.show_tooltip = true;
    }

    fn hide(&mut self) {
        self.show_tooltip = false;
    }
}

pub enum Msg {
    Show,
    Hide,
}

impl Component for Tooltip {
    type Message = Msg;
    type Properties = TooltipProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Tooltip {
            show_tooltip: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Show => {
                self.show();
            }
            Msg::Hide => {
                self.hide();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tooltip_style = if self.show_tooltip {
            "position: absolute; top: 100%; margin: auto; left: 0; right: 0; width: fit-content; display: block; background: green;"
        } else {
            "display: none; background: green;"
        };

        html! {
            <div class="tooltip-container">
                <span class="tooltip-trigger"
                    onmouseover={ctx.link().callback(|_| Msg::Show)}
                    onmouseout={ctx.link().callback(|_| Msg::Hide)}
                >
                    { for ctx.props().children.iter() }
                </span>
                <div class="tooltip" style={tooltip_style}>
                    { "This is a tooltip!" }
                </div>
            </div>
        }
    }
}
