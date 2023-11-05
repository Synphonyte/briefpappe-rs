use crate::i18n::locale_path;
use crate::types::Collection;
use crate::STATIC_URL;
use leptos::*;
use leptos_router::*;

#[component]
pub fn CollectionLink(collection: Collection) -> impl IntoView {
    let link_path = move || locale_path(&format!("/collections/{}", &collection.slug));

    let img_path = move |paper_path: &String| format!("{}{}/1.svg", STATIC_URL, paper_path);

    view! {
        <A href=link_path class="collection-link">
            <h2 class="title">{collection.title}</h2>
            {collection
                .papers
                .get(..5)
                .unwrap_or(&collection.papers)
                .iter()
                .map(|paper_path| view! { <img src=img_path(paper_path)/> })
                .collect::<Vec<_>>()}

        </A>
    }
}
