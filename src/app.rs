use crate::layouts::*;
use crate::routes::*;
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
                        <Route path="" view=Home />
                        <Route path="/collections/:slug" view=Collection />
                    </Route>
                    <Route path="/editor" view=EditorLayout>
                        <Route path="" view=move || view! {} />
                        <Route path="/:slug" view=Doc />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
