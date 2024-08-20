use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        body::Body as AxumBody,
        extract::State,
        response::IntoResponse,
        http::{Request, Response, StatusCode, Uri},
    };
    use axum::response::Response as AxumResponse;
    use tower::ServiceExt;
    use tower_http::services::ServeDir;
    use leptos::*;
    use crate::error_template::ErrorTemplate;
    use crate::error_template::AppError;

    pub async fn file_and_error_handler(
        uri: Uri,
        State(options): State<LeptosOptions>,
        req: Request<AxumBody>,
    ) -> AxumResponse {
        let root = options.site_root.clone();
        let res = get_static_file(uri.clone(), &root).await.unwrap();

        if res.status() == StatusCode::OK {
           res.into_response()
        } else {
            let mut errors = Errors::default();
            errors.insert_with_default_key(AppError::NotFound);
            let handler = leptos_axum::render_app_to_stream(options.to_owned(), move || view!{<ErrorTemplate outside_errors=errors.clone()/>});
            handler(req).await.into_response()
        }
    }

    async fn get_static_file(uri: Uri, root: &str) -> Result<Response<AxumBody>, (StatusCode, String)> {
        let req = Request::builder().uri(uri.clone()).body(AxumBody::empty()).unwrap();
        // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
        // This path is relative to the cargo root
        match ServeDir::new(root).oneshot(req).await {
            Ok(res) => Ok(res.into_response()),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {err}"),
            )),
        }
    }
}}
