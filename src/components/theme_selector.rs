use leptos::html::Button;
use leptos::*;
use leptos_use::{
    on_click_outside, use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};
use phosphor_leptos::{House, IconWeight, Monitor, MoonStars, Sun};

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
        ColorMode::Light => "#38bdf8".to_string(),
        ColorMode::Dark => "#38bdf8".to_string(),
        _ => "#94a3b8".to_string(),
    };

    view! {
        <div class="relative">
            <button
                class="flex h-6 w-6 items-center justify-center rounded-lg shadow-md shadow-black/5 ring-1 ring-black/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5"
                id="select-theme-button"
                node_ref=popup
                aria-expanded="false"
                aria-haspopup="true"
                on:click=toggle_popup
            >
                <div class="dark:hidden">
                    <Sun
                        color=Signal::derive(theme_button_color)
                        weight=IconWeight::Bold
                        size="16px"
                    />
                </div>
                <div class="hidden dark:block">
                    <MoonStars
                        color=Signal::derive(theme_button_color)
                        weight=IconWeight::Bold
                        size="16px"
                    />
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

                {
                    let active_icon_color = move || {
                        if mode.get() == ColorMode::Light {
                            "#38bdf8".to_string()
                        } else {
                            "#94a3b8".to_string()
                        }
                    };
                    let active_text_class = move || {
                        if mode.get() == ColorMode::Light {
                            "ml-3 text-sky-500"
                        } else {
                            "ml-3 text-slate-700 dark:text-slate-400"
                        }
                    };
                    move || {
                        view! {
                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Light)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                                    <Sun
                                        color=Signal::derive(active_icon_color)
                                        weight=IconWeight::Bold
                                        size="16px"
                                    />
                                </div>
                                <div class=active_text_class>"Light"</div>
                            </div>
                        }
                    }
                }

                {
                    let active_icon_color = move || {
                        if mode.get() == ColorMode::Dark {
                            "#38bdf8".to_string()
                        } else {
                            "#94a3b8".to_string()
                        }
                    };
                    let active_text_class = move || {
                        if mode.get() == ColorMode::Dark {
                            "ml-3 text-sky-500"
                        } else {
                            "ml-3 text-slate-700 dark:text-slate-400"
                        }
                    };
                    move || {
                        view! {
                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Dark)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                                    <MoonStars
                                        color=Signal::derive(active_icon_color)
                                        weight=IconWeight::Bold
                                        size="16px"
                                    />
                                </div>
                                <div class=active_text_class>"Dark"</div>
                            </div>
                        }
                    }
                }

                {
                    let active_icon_color = move || {
                        if mode.get() == ColorMode::Auto {
                            "#38bdf8".to_string()
                        } else {
                            "#94a3b8".to_string()
                        }
                    };
                    let active_text_class = move || {
                        if mode.get() == ColorMode::Auto {
                            "ml-3 text-sky-500"
                        } else {
                            "ml-3 text-slate-700 dark:text-slate-400"
                        }
                    };
                    move || {
                        view! {
                            <div
                                class="flex cursor-pointer select-none items-center rounded-[0.625rem] p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
                                on:click=move |_| set_mode.set(ColorMode::Auto)
                            >
                                <div class="rounded-md bg-white p-1 shadow ring-1 ring-slate-900/5 dark:bg-slate-700 dark:ring-inset dark:ring-white/5">
                                    <Monitor
                                        color=Signal::derive(active_icon_color)
                                        weight=IconWeight::Bold
                                        size="16px"
                                    />

                                </div>
                                <div class=active_text_class>"System"</div>
                            </div>
                        }
                    }
                }

            </div>
        </div>
    }
}
