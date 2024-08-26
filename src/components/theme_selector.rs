use leptos::html::Button;
use leptos::*;
use leptos_use::{
    on_click_outside, use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let popup = create_node_ref::<Button>();
    let (is_popup_open, set_popup_state) = create_signal(false);
    let _ = on_click_outside(popup, move |_| set_popup_state.set(false));
    let toggle_popup = move |_| set_popup_state.set(!is_popup_open.get_untracked());
    let popup_style = Signal::derive(move || {
        if is_popup_open() {
            ""
        } else {
            "display: none;"
        }
    });

    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().emit_auto(true));

    let theme_button_color = move || match mode() {
        ColorMode::Light => "#00adfa".to_string(),
        ColorMode::Dark => "#38bdf8".to_string(),
        _ => "#111a".to_string(),
    };

    let active_icon_color = move |color_mode| {
        if mode() == color_mode {
            "#38bdf8"
        } else {
            "#222a"
        }
    };
    let active_text_class = move |color_mode| {
        if mode() == color_mode {
            "ml-3 text-sky-500"
        } else {
            "ml-3 text-slate-700 dark:text-slate-400"
        }
    };

    view! {
        <div class="relative">
            <button
                class="flex h-6 w-6 items-center justify-center rounded-lg shadow-md shadow-black/5 ring-1 ring-black/5 bg-slate-200 dark:bg-slate-700 dark:ring-inset dark:ring-white/5"
                id="select-theme-button"
                node_ref=popup
                aria-expanded="false"
                aria-haspopup="true"
                on:click=toggle_popup
            >
                <div class="dark:hidden">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill=theme_button_color
                        viewBox="0 0 256 256"
                        >
                        <path d="M116,36V20a12,12,0,0,1,24,0V36a12,12,0,0,1-24,0Zm80,92a68,68,0,1,1-68-68A68.07,68.07,0,0,1,196,128Zm-24,0a44,44,0,1,0-44,44A44.05,44.05,0,0,0,172,128ZM51.51,68.49a12,12,0,1,0,17-17l-12-12a12,12,0,0,0-17,17Zm0,119-12,12a12,12,0,0,0,17,17l12-12a12,12,0,1,0-17-17ZM196,72a12,12,0,0,0,8.49-3.51l12-12a12,12,0,0,0-17-17l-12,12A12,12,0,0,0,196,72Zm8.49,115.51a12,12,0,0,0-17,17l12,12a12,12,0,0,0,17-17ZM48,128a12,12,0,0,0-12-12H20a12,12,0,0,0,0,24H36A12,12,0,0,0,48,128Zm80,80a12,12,0,0,0-12,12v16a12,12,0,0,0,24,0V220A12,12,0,0,0,128,208Zm108-92H220a12,12,0,0,0,0,24h16a12,12,0,0,0,0-24Z"></path>
                    </svg>
                </div>
                <div class="hidden dark:block">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill=theme_button_color
                        viewBox="0 0 256 256"
                    >
                        <path d="M244,96a12,12,0,0,1-12,12H220v12a12,12,0,0,1-24,0V108H184a12,12,0,0,1,0-24h12V72a12,12,0,0,1,24,0V84h12A12,12,0,0,1,244,96ZM144,60h4v4a12,12,0,0,0,24,0V60h4a12,12,0,0,0,0-24h-4V32a12,12,0,0,0-24,0v4h-4a12,12,0,0,0,0,24Zm75.81,90.38A12,12,0,0,1,222,162.3,100,100,0,1,1,93.7,34a12,12,0,0,1,15.89,13.6A85.12,85.12,0,0,0,108,64a84.09,84.09,0,0,0,84,84,85.22,85.22,0,0,0,16.37-1.59A12,12,0,0,1,219.81,150.38ZM190,172A108.13,108.13,0,0,1,84,66,76,76,0,1,0,190,172Z"></path>
                    </svg>
                </div>
            </button>

            <div
                class="absolute right-0 z-20 mt-3 w-36 space-y-1 origin-top-right rounded-xl bg-white p-3 text-sm shadow-md shadow-black/5 ring-1 ring-black/5 dark:bg-slate-800 dark:ring-white/5"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="select-theme-button"
                tabindex="-1"
                style=popup_style
            >
                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Light)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                        <svg // Sun
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            fill=move || active_icon_color(ColorMode::Light)
                            viewBox="0 0 256 256"
                            >
                                <path d="M116,36V20a12,12,0,0,1,24,0V36a12,12,0,0,1-24,0Zm80,92a68,68,0,1,1-68-68A68.07,68.07,0,0,1,196,128Zm-24,0a44,44,0,1,0-44,44A44.05,44.05,0,0,0,172,128ZM51.51,68.49a12,12,0,1,0,17-17l-12-12a12,12,0,0,0-17,17Zm0,119-12,12a12,12,0,0,0,17,17l12-12a12,12,0,1,0-17-17ZM196,72a12,12,0,0,0,8.49-3.51l12-12a12,12,0,0,0-17-17l-12,12A12,12,0,0,0,196,72Zm8.49,115.51a12,12,0,0,0-17,17l12,12a12,12,0,0,0,17-17ZM48,128a12,12,0,0,0-12-12H20a12,12,0,0,0,0,24H36A12,12,0,0,0,48,128Zm80,80a12,12,0,0,0-12,12v16a12,12,0,0,0,24,0V220A12,12,0,0,0,128,208Zm108-92H220a12,12,0,0,0,0,24h16a12,12,0,0,0,0-24Z"></path>
                            </svg>
                                </div>
                    <div class=move || active_text_class(ColorMode::Light)>"Light"</div>
                            </div>

                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Dark)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                        <svg // Moonstars
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            fill=move || active_icon_color(ColorMode::Dark)
                            viewBox="0 0 256 256"
                        >
                            <path d="M244,96a12,12,0,0,1-12,12H220v12a12,12,0,0,1-24,0V108H184a12,12,0,0,1,0-24h12V72a12,12,0,0,1,24,0V84h12A12,12,0,0,1,244,96ZM144,60h4v4a12,12,0,0,0,24,0V60h4a12,12,0,0,0,0-24h-4V32a12,12,0,0,0-24,0v4h-4a12,12,0,0,0,0,24Zm75.81,90.38A12,12,0,0,1,222,162.3,100,100,0,1,1,93.7,34a12,12,0,0,1,15.89,13.6A85.12,85.12,0,0,0,108,64a84.09,84.09,0,0,0,84,84,85.22,85.22,0,0,0,16.37-1.59A12,12,0,0,1,219.81,150.38ZM190,172A108.13,108.13,0,0,1,84,66,76,76,0,1,0,190,172Z"></path>
                        </svg>
                                </div>
                    <div class=move || active_text_class(ColorMode::Dark)>"Dark"</div>
                            </div>

                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Auto)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            fill=move || active_icon_color(ColorMode::Auto)
                            viewBox="0 0 256 256"
                        >
                            <path d="M208,36H48A28,28,0,0,0,20,64V176a28,28,0,0,0,28,28H208a28,28,0,0,0,28-28V64A28,28,0,0,0,208,36Zm4,140a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4Zm-40,52a12,12,0,0,1-12,12H96a12,12,0,0,1,0-24h64A12,12,0,0,1,172,228Z"></path>
                        </svg>
                                </div>
                    <div class=move || active_text_class(ColorMode::Auto)>"System"</div>
                            </div>
            </div>
        </div>
    }
}
