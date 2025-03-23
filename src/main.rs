use crate::components::Root;

use leptos::mount::mount_to_body;

mod components;
mod types;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Root);
}
