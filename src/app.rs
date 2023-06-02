use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::types::CollectionFilter;
use crate::server_functions::list_collections;
use crate::components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Briefpappe"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (filter, set_filter) = create_signal(cx, CollectionFilter::default());
    let filter_res = create_resource(cx, filter, list_collections);

    let papers_view = move || {
        filter_res.with(cx, |collections| {
            collections.clone().map(|collections| {
                collections.iter().map(|collection| {
                    view! { cx, <CollectionLink collection=collection.clone()/> }
                }).collect::<Vec<_>>()
            })
        })
    };

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading posts..."</p> }>
            <ul>{papers_view}</ul>
        </Suspense>
    }
}
