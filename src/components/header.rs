use super::ThemeSelector;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-50 flex flex-none flex-wrap items-center justify-between border-b-2 border-white p-4 shadow-md shadow-slate-900/5 transition duration-500 dark:shadow-none sm:px-6 lg:px-8">
            <div class="mr-6 flex lg:hidden">"<MobileNavigation/>"</div>
            <div class="relative flex flex-grow basis-0 items-center">
                <A href="/" class="hidden lg:block" attr:aria-label="Home page">
                    <div>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 fill-white/70 hover:fill-white" viewBox="0 0 256 256"><path d="M224,120v96a8,8,0,0,1-8,8H160a8,8,0,0,1-8-8V164a4,4,0,0,0-4-4H108a4,4,0,0,0-4,4v52a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8V120a16,16,0,0,1,4.69-11.31l80-80a16,16,0,0,1,22.62,0l80,80A16,16,0,0,1,224,120Z"></path></svg>
                    </div>
                </A>
            </div>
            <div class="relative flex basis-0 justify-end gap-6 sm:gap-8 md:flex-grow">
                <ThemeSelector />
            </div>
        </header>
    }
}
