use emissions_app_client::App;

// fn main() {
//     #[cfg(target_arch = "wasm32")]
//     wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
//     yew::Renderer::<App>::new().hydrate();
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
