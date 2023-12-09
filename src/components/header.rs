use super::ThemeSelector;
use leptos::*;
use leptos_router::*;
use phosphor_leptos::{House, IconWeight};

#[component]
pub fn Header(is_scrolled: Signal<bool>) -> impl IntoView {
    let header_class = move || {
        let default = "sticky top-0 z-50 flex flex-none flex-wrap items-center justify-between bg-white px-4 py-5 shadow-md shadow-slate-900/5 transition duration-500 dark:shadow-none sm:px-6 lg:px-8";
        if is_scrolled() {
            format!("{default} dark:bg-slate-900/95 dark:backdrop-blur dark:[@supports(backdrop-filter:blur(0))]:bg-slate-900/75")
        } else {
            format!("{default} dark:bg-transparent")
        }
    };

    view! {
        <header class=header_class>
            <div class="mr-6 flex lg:hidden">"<MobileNavigation/>"</div>
            <div class="relative flex flex-grow basis-0 items-center">
                <A href="/" class="hidden lg:block" attr:aria-label="Home page">
                    <div class="dark:hidden">
                        <House color="#334155" weight=IconWeight::Fill size="24px"/>
                    </div>
                    <div class="hidden dark:block">
                        <House color="#e0f2fe" weight=IconWeight::Fill size="24px"/>
                    </div>
                </A>
            </div>
            <div class="-my-5 mr-6 sm:mr-8 md:mr-0">"<Search/>"</div>
            <div class="relative flex basis-0 justify-end gap-6 sm:gap-8 md:flex-grow">
                <ThemeSelector />
            </div>
        </header>
    }
}
