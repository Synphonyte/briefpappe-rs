use crate::components::CollectionLink;
use crate::server_functions::list_collections;
use crate::types::CollectionFilter;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
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
