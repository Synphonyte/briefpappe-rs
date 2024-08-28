use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full flex justify-center mt-8 py-8 bg-black/70 text-gray-300">
            <p>"This page and all of its contents are only for private, non-commercial use. If you want to contribute designs or translations or found a bug please contact us at "
                <a class="text-teal-300 border-b border-indigo-300" href="mailto:briefpappe@gmail.com">"briefpappe@gmail.com"</a>
            </p>
        </footer>
    }
}
