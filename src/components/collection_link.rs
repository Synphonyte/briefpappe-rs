use leptos::*;
use leptos_router::*;
use crate::types::Collection;
use crate::i18n::locale_path;
use crate::STATIC_URL;

#[component]
pub fn CollectionLink(cx: Scope, collection: Collection) -> impl IntoView {
    let link_path = move || locale_path(cx, &format!("/collections/{}", &collection.slug));

    let img_path = move |paper_path: &String| {
        format!("{}{}/1.svg", STATIC_URL, paper_path)
    };

    view! { cx,
        <A href=link_path class="collection-link">
            <h2 class="title">{ collection.title }</h2>
            { collection.papers.get(..5).unwrap_or(&collection.papers)
                .iter()
                .map(|paper_path|view! { cx,<img src=img_path(paper_path)/> })
                .collect::<Vec<_>>()
            }
        </A>
    }
}