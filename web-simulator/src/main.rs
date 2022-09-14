mod app;
mod borrowed;
mod http;
mod mqtt;
mod publisher;
mod utils;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
