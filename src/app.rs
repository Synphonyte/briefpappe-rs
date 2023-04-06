use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::types::{Paper, PaperFilter};
use crate::server_functions::list_papers;

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
        <Title text="Welcome to Leptos"/>

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
    let (filter, set_filter) = create_signal(cx, PaperFilter::default());
    let filter_res = create_resource(cx, filter, list_papers);

    let papers_view = move || {
        filter_res.with(cx, |papers| {
            papers.clone().map(|papers| {
                papers.iter().map(|paper| {
                    view! { cx, <PaperView paper=paper.clone()/> }
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

#[component]
fn PaperView(cx: Scope, paper: Paper) -> impl IntoView {
    view! { cx,
        <li>
            <h2>{ paper.title }</h2>
            <p>{ paper.tags.join(", ") }</p>
        </li>
    }
}