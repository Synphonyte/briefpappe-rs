use crate::components::{Header, Navigation};
use leptos::html::Div;
use leptos::*;
use leptos_router::Outlet;
use leptos_use::{use_scroll, UseScrollReturn};

#[component]
pub fn AppLayout() -> impl IntoView {
    let element = create_node_ref::<Div>();

    let UseScrollReturn { arrived_state, .. } = use_scroll(element);

    let is_scrolled = move || arrived_state().top;

    view! {
        <div id="app" node_ref=element class="flex min-h-full bg-white dark:bg-slate-900">
            <div class="flex w-full flex-col">
                <Header is_scrolled=Signal::derive(is_scrolled)/>
                // <Hero /> // Todo: only if is homepage
                <div class="relative mx-auto flex w-full max-w-8xl flex-auto justify-center sm:px-2 lg:px-8 xl:px-12">
                    <div class="hidden lg:relative lg:block lg:flex-none">
                        <div class="absolute inset-y-0 right-0 w-[50vw] bg-slate-50 dark:hidden"></div>
                        <div class="absolute bottom-0 right-0 top-16 hidden h-12 w-px bg-gradient-to-t from-slate-800 dark:block"></div>
                        <div class="absolute bottom-0 right-0 top-28 hidden w-px bg-slate-800 dark:block"></div>
                        <div class="sticky top-[4.75rem] -ml-0.5 h-[calc(100vh-4.75rem)] w-64 overflow-y-auto overflow-x-hidden py-16 pl-0.5 pr-8 xl:w-72 xl:pr-16">
                            <Navigation/>
                        </div>
                    </div>
                    <div class="min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:pl-8 lg:pr-0 xl:px-16">
                        <Outlet/>
                    </div>
                </div>
            </div>
        </div>
    }
}
