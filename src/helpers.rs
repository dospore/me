use web_sys::{ window, Window };

pub fn get_window_size() -> Option<(u32, u32)> {
    let window: Window = window().unwrap();
    if let Some(document) = window.document() {
        if let Some(document_element) = document.document_element() {
            let width = document_element.client_width();
            let height = document_element.client_height();
            return Some((width as u32, height as u32));
        }
    }

    None
}
