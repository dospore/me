use yew_router::prelude::*;
use web_sys::window;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    // pub position: Position,
    pub height: i32,
    pub width: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/collection")]
    Collection,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum ScreenType {
    Desktop,
    Mobile
}

pub fn get_screen_type() -> ScreenType {
    let window = window().expect("should have a window in this context");
    let width = window.inner_width().expect("should have width").as_f64().unwrap_or(0.0);

    if width > 768.0 {
        ScreenType::Desktop
    } else {
        ScreenType::Mobile
    }
}
