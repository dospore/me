use gloo::timers::callback::Interval;
use web_sys::MouseEvent;
// use std::collections::LinkedList;
use gloo_console::log;
use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    Rng,
};
use yew::prelude::*;
use crate::utils::{Grid, Position};
use crate::helpers::get_window_size;

#[derive(Debug, Copy, Clone)]
enum LineType {
    Up,
    Down,
    Right, 
    Left, 
}

impl Distribution<LineType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LineType {

        match rng.gen_range(0..=3) { // rand 0.8
            0 => LineType::Up,
            1 => LineType::Down,
            2 => LineType::Right,
            _ => LineType::Left,
        }
    }
}

impl LineType {
    pub fn get_weight(l: LineType, p: Position, g: Grid, mouse_position: Option<Position>) -> i32 {
        let (weight_x, weight_y) = match mouse_position {
            Some(target) => {
                let distance_x = (target.x - p.x).abs();
                let distance_y = (target.y - p.y).abs();


                let weight_y: i32 = ((distance_y as f64 / g.height as f64).clamp(0.0, 1.0) * 100.0) as i32;
                let weight_x: i32 = ((distance_x as f64 / g.width as f64).clamp(0.0, 1.0) * 100.0) as i32;
                if weight_y == 0 && weight_x == 0 {
                    (1, 1)
                } else {
                    (weight_x, weight_y)
                }
            }, _ => {
                (1, 1)
            }
        };

        match l {
            LineType::Up => {
                match p.y < 0 {
                    true => 0,
                    _ => weight_y
                }
            },
            LineType::Down => {
                match p.y + Line::LENGTH > g.height {
                    true => 0,
                    _ => weight_y
                }
            },
            LineType::Right => {
                match p.x + Line::WIDTH > g.width {
                    true => 0,
                    _ => weight_x
                }
            },
            LineType::Left => {
                match p.x < 0 {
                    true => 0,
                    _ => weight_x
                }
            },
        }
    }

    pub fn random(p: Position, g: Grid, mouse_position: Option<Position>) -> LineType {
        let mut rng = rand::thread_rng();
        let items = [LineType::Up, LineType::Down, LineType::Left, LineType::Right].map(|t| (t, LineType::get_weight(t, p, g, mouse_position)));
        // let items = [(LineType::Up, LineType::get_weight(LineType::Up, p)), (LineType::Down, 3), (LineType::Right, 7), (LineType::Left, 0)];
        let weighted_dist= WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        items[weighted_dist.sample(&mut rng)].0
    }

}

#[derive(Debug)]
struct Line {
    position: Position,
    t: LineType
}

impl Line {
    const WIDTH: i32 = 2;
    const LENGTH: i32 = 20;

    pub fn render(&self) -> Html {
        let Position { x, y } = self.position;
        let line_style = match self.t {
            LineType::Up => {
                format!("left: {}px; top: {}px; height: {}px; width: {}px;", x, y - Line::LENGTH, Line::LENGTH, Line::WIDTH)
            },
            LineType::Down => {
                format!("left: {}px; top: {}px; height: {}px; width: {}px;", x, y, Line::LENGTH, Line::WIDTH)
            }
            LineType::Right => {
                format!("left: {}px; top: {}px; width: {}px; height: {}px;", x, y, Line::LENGTH, Line::WIDTH)
            }
            _ => {
                format!("left: {}px; top: {}px; width: {}px; height: {}px;", x - Line::LENGTH, y, Line::LENGTH, Line::WIDTH)
            }
        };
        html! {
            <div key={format!("{}{}{:?}", self.position.x, self.position.y, self.t)} class="line" style={line_style} />
        }
    }

}

#[derive(PartialEq)]
pub enum Side {
    Left, 
    Right
}

#[derive(Properties, PartialEq)]
pub struct LinesProps {
    pub side: Side,
    #[prop_or_default]
    pub children: Children,
}


pub enum Msg {
    Tick,
    MouseMove(MouseEvent),
    // StartDrawing,
    // StopDrawing
}

#[derive(Debug)]
pub struct Lines {
    position: Position, // drawing position
    mouse_position: Option<Position>,
    grid: Grid, // grid dimensions
    lines: Vec<Line>,
    _interval: Interval,
    on_mouse_move: Callback<MouseEvent>
}

impl Component for Lines {
    type Message = Msg;
    type Properties = LinesProps;


    fn create(ctx: &Context<Self>) -> Self {
        let (grid_width, grid_height) = match get_window_size() {
            Some((w, h)) => (w - 15, h - 20),
            _ => (0, 0)
        };

        let lines = Vec::new();
        let _interval = {
            let link = ctx.link().clone();
            Interval::new(20, move || {
                link.send_message(Msg::Tick)
            })
        };

        // starting drawing position
        let (x, y) = match ctx.props().side {
            Side::Right => (grid_width, grid_height),
            Side::Left => (0, 0),
        };

        Self {
            grid: Grid {
                width: grid_width as i32,
                height: grid_height as i32,
            },
            position: Position {
                x: x as i32,
                y: y as i32,
            },
            lines,
            _interval,
            mouse_position: None,
            on_mouse_move: ctx.link().callback(|e| Msg::MouseMove(e))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                    let line_type: LineType = LineType::random(self.position, self.grid, self.mouse_position);
                    let new_line = Line {
                        position: Position {
                            x : self.position.x,
                            y : self.position.y,
                        },
                        t: line_type
                    };

                    self.lines.push(new_line);

                    match line_type {
                        LineType::Up => {
                            self.position.y -= Line::LENGTH;
                        },
                        LineType::Down => {
                            self.position.y += Line::LENGTH;
                        },
                        LineType::Right => {
                            self.position.x += Line::LENGTH - Line::WIDTH;
                        },
                        _ => {
                            self.position.x -= Line::LENGTH;
                        }
                    }
                    true
                // }
            }, Msg::MouseMove(event) => {
                let x = event.client_x();
                let y = event.client_y();
                self.mouse_position = Some(Position { x, y });
                log!("{}, {}", x, y);
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let side = match ctx.props().side {
            Side::Left => "left",
            Side::Right => "right"
        };
        html! {
            <div class={classes!(side, "lines-container")} onmousemove={&self.on_mouse_move}>
                { for self.lines.iter().map(Line::render) }
            </div>
        }
    }
}
