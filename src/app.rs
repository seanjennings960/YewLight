use yew::{html, Component, ComponentLink, Html, ShouldRender};
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
    backend: String,
}

pub enum Mode {
    On,
    Off,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::On => "on",
            Mode::Off => "off",
        }.to_string()
    }
}

pub enum Msg {
    Light(Mode),
    Update(String),
    Error(String),
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
        let backend = "http://192.168.0.102:3030".to_string();
        App { value, fetch_value, fetcher, console, link, task, backend }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Light(mode) => {
                let url = format!("{}/light/{}", self.backend, mode.to_string());
                let request = Request::post(url)
                    .body(Nothing)
                    .expect("Failed to build request");

                let callback = self.link.callback(
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
                                    Msg::Error(meta.status.to_string())
                                }
                            },
                            Err(e) => {
                                console.log("Error in response!!");
                                Msg::Error(format!(
                                    "Error in response: {:?}", e.backtrace()))
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
            Msg::Error(resp) => {
                self.fetch_value = format!("Error fetching from backend: {}", resp);
                true
            }
        }
    }
    fn view(&self) -> Html {
        let turn_on = self.link.callback(|_| Msg::Light(Mode::On));
        let turn_off = self.link.callback(|_| Msg::Light(Mode::Off));
        html! {
            <div class="LightsWrapper">
                <section class="lights">
                    <header class="header">
                        <h1>{ "Lights!" }</h1>
                    </header>
                </section>
                <section class="button1">
                    <button onclick=turn_on >{ "Turn me on!" }</button>
                </section>
                <section class="button2">
                    <button onclick=turn_off >{ "Turn me off!" }</button>
                </section>
                <section class="output">
                    <p>{ self.view_output() }</p>
                </section>
            </div>
        }
    }
}


impl App {

    fn view_output(&self) -> Html {
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
