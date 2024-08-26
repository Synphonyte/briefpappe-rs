use cfg_if::cfg_if;
pub mod app;

mod components;
pub mod error_template;
pub mod fileserv;
mod i18n;
pub mod layouts;
mod routes;
pub mod server_functions;

#[cfg(feature = "ssr")]
pub mod state;
pub mod types;

pub const STATIC_URL: &str = "https://pepe.cces.ch/";

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}
