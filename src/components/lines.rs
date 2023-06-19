use gloo::timers::callback::Interval;
use web_sys::MouseEvent;
use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    Rng,
};
use yew::prelude::*;
use crate::utils::{Grid, Position};
use crate::helpers::get_window_size;
// use gloo_console::log;
// use gloo::utils::window;
// use gloo::events::EventListener;

#[derive(Debug, Copy, Clone)]
pub enum LineType {
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

pub fn get_weights(p: &Position, g: Grid, mouse_position: Option<Position>) -> [(LineType, i32); 4] {
    let mut up_weight: i32 = 1;
    let mut down_weight: i32 = 1;
    let mut left_weight: i32 = 1;
    let mut right_weight: i32 = 1;

    // UP 
    if p.y < 0 {
        up_weight = 0;
    }

    // DOWN
    if p.y + Line::LENGTH > g.height {
        down_weight = 0;
    }

    // RIGHT
    if p.x + Line::WIDTH > g.width {
        right_weight = 0;
    }

    // LEFT
    if p.x < 0 {
        left_weight = 0;
    }

    if let Some(target) = mouse_position {
        let distance_x = (target.x - p.x).abs();
        let distance_y = (target.y - p.y).abs();


        let weight_y: i32 = ((distance_y as f64 / g.height as f64).clamp(0.0, 1.0) * 100.0) as i32;
        let weight_x: i32 = ((distance_x as f64 / g.width as f64).clamp(0.0, 1.0) * 100.0) as i32;

        let randomness = 5;
        // favour moving towards target but keep some randomness
        if p.y < target.y {
            down_weight = std::cmp::max(weight_y - randomness, randomness);
            up_weight = randomness;
        } else if p.y > target.y {
            up_weight = std::cmp::max(weight_y - randomness, randomness);
            down_weight = randomness;
        }

        if p.x < target.x {
            right_weight = std::cmp::max(weight_x - randomness, randomness);
            left_weight = randomness;
        } else if p.x > target.x {
            left_weight = std::cmp::max(weight_x - randomness, randomness);
            right_weight = randomness;
        }
    }

    return [
        (LineType::Right, right_weight),
        (LineType::Left, left_weight),
        (LineType::Up, up_weight),
        (LineType::Down, down_weight)
    ]
}

impl LineType {
    pub fn random(p: &Position, g: Grid, mouse_position: Option<Position>) -> LineType {
        let mut rng = rand::thread_rng();
        let items = get_weights(p, g, mouse_position);
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

#[derive(Properties, PartialEq)]
pub struct LinesProps {
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
    mouse_position: Option<Position>,
    grid: Grid, // grid dimensions
    positions: Vec<Position>, // drawing position
    lines: Vec<Line>,
    on_mouse_move: Callback<MouseEvent>,
    _interval: Interval,
}

impl Component for Lines {
    type Message = Msg;
    type Properties = LinesProps;


    fn create(ctx: &Context<Self>) -> Self {
        let (grid_width, grid_height) = match get_window_size() {
            Some((w, h)) => (w - 15, h - 20),
            _ => (0, 0)
        };

        let _interval = {
            let link = ctx.link().clone();
            Interval::new(20, move || {
                link.send_message(Msg::Tick)
            })
        };

        let lines = Vec::new();

        // could make this dynamic
        let positions: Vec<Position> = vec![
            Position { x: grid_width as i32, y: grid_height as i32 },
            Position { x: 0, y: 0 }
        ];

        // let listener = EventListener::new(&window(), "storage", move |_| state_storage_changed.set(true));


        Self {
            grid: Grid {
                width: grid_width as i32,
                height: grid_height as i32,
            },
            positions,
            lines,
            _interval,
            mouse_position: None,
            on_mouse_move: ctx.link().callback(|e| Msg::MouseMove(e))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                for position in self.positions.iter_mut() {
                    let line_type: LineType = LineType::random(&position, self.grid, self.mouse_position);
                    let new_line = Line {
                        position: Position {
                            x : position.x,
                            y : position.y,
                        },
                        t: line_type
                    };

                    self.lines.push(new_line);

                    match line_type {
                        LineType::Up => {
                            position.y -= Line::LENGTH;
                        },
                        LineType::Down => {
                            position.y += Line::LENGTH;
                        },
                        LineType::Right => {
                            position.x += Line::LENGTH - Line::WIDTH;
                        },
                        _ => {
                            position.x -= Line::LENGTH;
                        }
                    }
                }
                true
            }, Msg::MouseMove(event) => {
                let x = event.client_x();
                let y = event.client_y();
                self.mouse_position = Some(Position { x, y });
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("lines-container")} onmousemove={&self.on_mouse_move}>
                { for self.lines.iter().map(Line::render) }
            </div>
        }
    }
}





