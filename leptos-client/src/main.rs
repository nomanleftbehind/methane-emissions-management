use leptos::*;
use methane_emissions_management_leptos_client::routes::RouterExample;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(RouterExample)
}
