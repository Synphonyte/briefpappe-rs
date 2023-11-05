cfg_if::cfg_if!{ if #[cfg(feature = "ssr")] {
    use axum::extract::State;
    use briefpappe_rs::state::AppState;
    use http::Request;
    use axum::body::Body as AxumBody;
    use axum::response::{IntoResponse, Response};
    use leptos::*;
    use briefpappe_rs::app::App;

    pub async fn leptos_routes_handler(
        State(app_state): State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_app_to_stream_with_context(
            app_state.leptos_options.clone(),
            move || {
            },
            || view! { <App/> },
        );
        handler(req).await.into_response()
    }


    #[tokio::main]
    async fn main() {
        use axum::{routing::post, Router};
        use briefpappe_rs::app::*;
        use briefpappe_rs::fileserv::file_and_error_handler;
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use log::info;

        simple_logger::init_with_level(log::Level::Warn).expect("couldn't initialize logging");

        // Setting get_configuration(None) means we'll be using cargo-leptos's env values
        // For deployment these variables are:
        // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
        // Alternately a file can be specified such as Some("Cargo.toml")
        // The file would need to be included with the executable when moved to deployment
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(App);

        let app_state = AppState {
            leptos_options: leptos_options.clone(),
        };
        // build our application with a route
        let app = Router::new()
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .leptos_routes_with_handler(routes, leptos_routes_handler)
            .fallback(file_and_error_handler)
            .with_state(app_state);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        info!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
} else {
    pub fn main() {
        // no client-side main function
        // unless we want this to work with e.g., Trunk for a purely client-side app
        // see lib.rs for hydration function instead
    }

}}