use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};
use yew::format::Nothing;
// use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use http::Request;
use failure::Error;

pub struct App {
    value: i32,
    fetch_value: String,
    fetcher: FetchService,
    console: ConsoleService,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    DoIt,
    Fetch,
    Update(String),
    Error,
}

impl Component for App {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = 0;
        let fetch_value = "".to_string();
        let fetcher = FetchService::new();
        let console = ConsoleService::new();
        let task = None;
        App { value, fetch_value, fetcher, console, link, task }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                self.value = self.value + 1;
                self.console.log(
                    &format!("Button pressed for {:?} time.", self.value));
                println!("Button pressed for {:?} time.", self.value);
                true
            }
            Msg::Fetch => {
                let request = Request::get("http://localhost:3030/hello/warp")
                    .body(Nothing)
                    .expect("Failed to build request");
                // let request = Request::new("Hello".to_string());

                // (|response: Response<Result<String, failure::Error>>| {
                let callback = self.link.send_back(
                    |response: Response<Result<String, Error>>| {
                        let mut console = ConsoleService::new();
                        console.log("Hello fetching!!");
                        let (meta, result) = response.into_parts();
                        match result {
                            Ok(body) => {
                                if meta.status.is_success() {
                                    Msg::Update(body)
                                } else {
                                    console.log(&format!(
                                        "Error in status: {:?}", meta.status));
                                    Msg::Error
                                }
                            },
                            Err(_) => {
                                console.log("Error in response!!");
                                Msg::Error
                            },
                        }
                    }
                );
                self.console.log("Creating fetch task.");
                self.task = Some(self.fetcher.fetch(request, callback));
                false
            }
            Msg::Update(resp) => {
                self.fetch_value = format!("Fetched string: {}", resp);
                true
            }
            Msg::Error => {
                self.fetch_value = "Error fetching from backend.".to_string();
                true
            }
        }
    }
}


impl Renderable<App> for App {
    fn view(&self) -> Html<App> {
        // let onclick = self.link.callback(|_| Msg::DoIt);
        html! {
            <div class="LightsWrapper">
                <section class="lights">
                    <header class="header">
                        <h1>{ "Lights!" }</h1>
                    </header>
                </section>
                <section class="button1">
                    <button onclick=|_| Msg::DoIt >{ "Turn me on!" }</button>
                </section>
                <section class="button2">
                    <button onclick=|_| Msg::Fetch >{ "Fetch from server." }</button>
                </section>
                <section class="output">
                    <p>{ self.view_output() }</p>
                </section>
            </div>
        }
    }
}

impl App {

    fn view_output(&self) -> Html<App> {
        let mut output = "Count: ".to_string();
        output.push_str(&self.value.to_string());
        let fetch_output = &self.fetch_value;

        html! {
            <div>
                <p> {output} </p>
                <p> {fetch_output} </p>
            </div>
        }
    }
}

// fn main() {
//     yew::start_app::<App>();
// }
