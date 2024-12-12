pub mod app;
pub mod component;
pub mod model;
pub mod repository;
pub mod shared;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
