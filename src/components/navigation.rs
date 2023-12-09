use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug)]
pub struct NavigationLink {
    pub title: &'static str,
    pub href: &'static str,
}

#[derive(Clone, Debug)]
pub struct NavigationSection {
    pub title: &'static str,
    pub links: Vec<NavigationLink>,
}

#[derive(Clone, Debug)]
pub struct Navigation {
    pub sections: Vec<NavigationSection>,
}

#[component]
pub fn Navigation() -> impl IntoView {
    let navigation = vec![
        NavigationSection {
            title: "Section1",
            links: Vec::from([
                NavigationLink {
                    title: "Nav-Link1",
                    href: "/",
                },
                NavigationLink {
                    title: "Nav-Link2",
                    href: "/getting-started",
                },
                NavigationLink {
                    title: "Nav-Link3",
                    href: "/quickstart",
                },
            ]),
        },
        NavigationSection {
            title: "Section2",
            links: Vec::from([NavigationLink {
                title: "Nav-Link1",
                href: "/guide",
            }]),
        },
        NavigationSection {
            title: "Section3",
            links: Vec::from([NavigationLink {
                title: "Nav-Link1",
                href: "/api",
            }]),
        },
    ];

    view! {
        <nav class="text-base lg:text-sm">
            <ul role="list" class="space-y-9">
                {navigation
                    .into_iter()
                    .map(|section| {
                        view! {
                            <li>
                                <h2 class="font-display font-medium text-slate-900 dark:text-white">
                                    {section.title}
                                </h2>
                                <ul
                                    role="list"
                                    class="mt-2 space-y-2 border-l-2 border-slate-100 dark:border-slate-800 lg:mt-4 lg:space-y-4 lg:border-slate-200"
                                >
                                    {section
                                        .links
                                        .into_iter()
                                        .map(|link| {
                                            view! {
                                                <li class="relative">
                                                    <A
                                                        href=link.href
                                                        active_class="font-semibold text-sky-500 before:bg-sky-500"
                                                        class="block w-full pl-3.5 before:pointer-events-none before:absolute before:-left-1 before:top-1/2 before:h-1.5 before:w-1.5 before:-translate-y-1/2 before:rounded-full text-slate-500 before:hidden before:bg-slate-300 hover:text-slate-600 hover:before:block dark:text-slate-400 dark:before:bg-slate-700 dark:hover:text-slate-300"
                                                    >
                                                        {link.title}
                                                    </A>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </nav>
    }
}
