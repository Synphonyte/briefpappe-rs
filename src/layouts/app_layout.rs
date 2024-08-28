use crate::components::{Footer, Header, Navigation};
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div id="app" class="flex min-h-full bg-[url('/letter-bg.svg')] bg-gray-300 dark:bg-slate-900">
            <div class="flex w-full flex-col">
                <Header />
                <div class="relative mx-auto flex w-full max-w-screen-2xl flex-auto justify-center sm:px-2 lg:px-4">
                    <div class="hidden lg:relative lg:block lg:flex-none">
                        <div class="absolute inset-y-0 right-0 w-[50vw] bg-slate-50/25 dark:hidden"></div>
                        <div class="absolute bottom-0 right-0 top-16 hidden h-12 w-px bg-gradient-to-t from-slate-800 dark:block"></div>
                        <div class="absolute bottom-0 right-0 top-28 hidden w-px bg-slate-800 dark:block"></div>
                        <div class="sticky top-[4.75rem] -ml-0.5 h-[calc(100vh-4.75rem)] w-48 overflow-y-auto overflow-x-hidden py-16 pl-0.5 pr-8 xl:w-56 xl:pr-16">
                            <Navigation/>
                        </div>
                    </div>
                    <div class="min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:pl-8 lg:pr-0 xl:px-16">
                        <Outlet/>
                    </div>
                </div>
                <Footer />
            </div>
        </div>
    }
}
