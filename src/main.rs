use console_error_panic_hook;
use leptos::mount::mount_to_body;
mod Components;
mod app;
use app::App;
fn main() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
}
