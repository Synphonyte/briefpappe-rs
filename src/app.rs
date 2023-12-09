use crate::components::*;
use crate::layouts::AppLayout;
use crate::server_functions::list_collections;
use crate::types::CollectionFilter;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/briefpappe_rs.css"/>

        // sets the document title
        <Title text="Briefpappe"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=AppLayout>
                        <Route path="" view=HomePage />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (filter, set_filter) = create_signal(CollectionFilter::default());
    let filter_res = create_resource(filter, list_collections);

    let papers_view = move || {
        filter_res.with(|collections| {
            let collections = collections.clone();
            collections.map(|collections| {
                collections.map(|collections| {
                    collections
                        .iter()
                        .map(|collection| {
                            view! { <CollectionLink collection=collection.clone()/> }
                        })
                        .collect::<Vec<_>>()
                })
            })
        })
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>{papers_view}</ul>
        </Suspense>
    }
}
