use crate::components::{Header, LeftSideBar, RightSideBar};
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn EditorLayout() -> impl IntoView {
    view! {
        <div
            id="app"
            class="flex min-h-full bg-white dark:bg-slate-900"
        >
            <div class="flex w-full flex-col">
                <Header/>
                <div class="relative mx-auto flex w-full max-w-screen-2xl flex-auto justify-center bg-gray-400">
                    <LeftSideBar/>
                    <Outlet/>
                    <RightSideBar/>
                </div>
            </div>
        </div>
    }
}
