mod app;
use app::Root;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}
