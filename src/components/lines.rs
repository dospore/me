use gloo::timers::callback::Interval;
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
    pub fn get_weight(l: LineType, p: Position, g: Grid) -> i32 {
        log!(g.width, g.height);
        match l {
            LineType::Up => {
                match p.y < 0 {
                    true => 0,
                    _ => 2
                }
            },
            LineType::Down => {
                match p.y + Line::LENGTH > g.height {
                    true => 0,
                    _ => 2
                }
            },
            LineType::Right => {
                match p.x + Line::WIDTH > g.width {
                    true => 0,
                    _ => 2
                }
            },
            LineType::Left => {
                match p.x < 0 {
                    true => 0,
                    _ => 2
                }
            },
        }
    }

    pub fn random(p: Position, g: Grid) -> LineType {
        let mut rng = rand::thread_rng();
        let items = [LineType::Up, LineType::Down, LineType::Left, LineType::Right].map(|t| (t, LineType::get_weight(t, p, g)));
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

#[derive(Debug)]
pub struct Lines {
    position: Position, // drawing position
    grid: Grid, // grid dimensions
    lines: Vec<Line>,
    _interval: Interval,
}


pub enum Msg {
    Tick,
    // StartDrawing,
    // StopDrawing
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
            _interval 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                    let line_type: LineType = LineType::random(self.position, self.grid);
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
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let side = match ctx.props().side {
            Side::Left => "left",
            Side::Right => "right"
        };
        html! {
            <div class={classes!(side, "lines-container")}>
                { for self.lines.iter().map(Line::render) }
            </div>
        }
    }
}
