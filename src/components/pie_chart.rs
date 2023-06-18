use gloo::console::log;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window,
    Blob,
    // ImageBitmap,
    RequestInit,
    HtmlCanvasElement,
    CanvasRenderingContext2d,
    Request, Response
};

use yew::prelude::*;
pub enum Msg {
    FetchOk(Vec<f64>),
    FetchFail(SomeError),
    Render,
}

pub struct SomeError {
    err: JsValue,
}

impl From<JsValue> for SomeError {
    fn from(v: JsValue) -> Self {
        Self { err: v }
    }
}

pub async fn fetch_data(user: &str) -> Result<Vec<f64>, SomeError> {
    let base_url = "https://api.github.com";

  // async getGithubData(username) {
    // const responseUser = await fetch(this.url + username);
    // const responseRepo = await fetch(this.url + username + "/repos");

    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(format!("{base_url}/users/{user}", base_url=base_url, user=user).as_str(), &opts)?;
    let window = window().unwrap();
    let resp_js_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_js_value.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    // let blob: Blob = JsFuture::from(resp.blob()?).await?.dyn_into()?;
    log!(json);

    // let image_bitmap_prom = window.create_image_bitmap_with_blob(&blob)?;
    // Ok(JsFuture::from(image_bitmap_prom).await?.dyn_into()?)
    //
    //
    Ok(vec![30.0, 20.0, 50.0])
}

pub struct PieChart {
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
}

impl Component for PieChart {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match fetch_data("dospore").await {
                Ok(image) => Msg::FetchOk(image),
                Err(err) => Msg::FetchFail(err),
            }
        });
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(Msg::Render)) as Box<dyn FnMut()>);
        Self {
            canvas: NodeRef::default(),
            callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchOk(data) => {
                // let width: usize = image.width().try_into().unwrap();
                // let height: usize = image.height().try_into().unwrap();
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                canvas.set_width(300);
                canvas.set_height(300);
                let context: CanvasRenderingContext2d =
                    canvas.get_context("2d").unwrap().unwrap().unchecked_into();

                // Clear the canvas
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                // Draw the pie chart
                let center_x = canvas.width() as f64 / 2.0;
                let center_y = canvas.height() as f64 / 2.0;
                let radius = (canvas.width().min(canvas.height()) / 2) as f64 * 0.8;


                let total: f64 = data.iter().sum();
                let mut start_angle = 0.0;

                let colors = vec!["#0000ff", "#ff0000", "#00ff00"];
                let mut i = 0;

                for value in data {
                    let slice_angle = (value / total) * 2.0 * std::f64::consts::PI;
                    context.begin_path();
                    context.arc(center_x, center_y, radius, start_angle, start_angle + slice_angle)
                        .expect("Failed to draw arc");
                    context.line_to(center_x, center_y);
                    context.set_fill_style(&JsValue::from_str(colors[i])); // Example color
                    context.fill();
                    context.set_stroke_style(&JsValue::from_str("#ffffff")); // Example stroke color
                    context.stroke();
                    context.close_path();

                    start_angle += slice_angle;
                    i = i + 1;
                }
                ctx.link().send_message(Msg::Render);
                true
            }
            Msg::FetchFail(e) => {
                log!(e.err);
                true
            }
            Msg::Render => {
                self.render();
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas
                    id="canvas"
                    ref={self.canvas.clone()}>
                </canvas>

                <script crossorigin="anonymous" mode="cors" src="https://github-readme-stats.vercel.app/api/top-langs/?username=anuraghazra"></script>
            </div>
        }
    }
}

impl PieChart {
    fn render(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let context: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        // ctx.set_global_alpha(0.05);
        // ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
        // ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        context.set_global_alpha(0.5);
        // let map = &self.brightness_map;
        // self.particles.iter_mut().for_each(|particle| {
            // particle.update(map);
            // let x = particle.x as usize;
            // let y = particle.y as usize;
            // let (_r, _g, _b, brightness) = map[y][x];
            // ctx.set_global_alpha(brightness * 0.3); // higher velocity higher alpha
            // particle.render(&ctx, map);
        // });
        // for value in data {
            // let slice_angle = (value / total) * 2.0 * std::f64::consts::PI;
            // context.begin_path();
            // context.arc(center_x, center_y, radius, start_angle, start_angle + slice_angle)
                // .expect("Failed to draw arc");
            // context.line_to(center_x, center_y);
            // context.set_fill_style(&JsValue::from_str(colors[i])); // Example color
            // context.fill();
            // context.set_stroke_style(&JsValue::from_str("#ffffff")); // Example stroke color
            // context.stroke();
            // context.close_path();

            // start_angle += slice_angle;
            // i = i + 1;
        // }
        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}
