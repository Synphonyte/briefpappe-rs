use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="overflow-hidden bg-slate-900 dark:-mb-32 dark:mt-[-4.75rem] dark:pb-32 dark:pt-[4.75rem]">
            <div class="py-16 sm:px-2 lg:relative lg:px-0 lg:py-20">
                <div class="mx-auto grid max-w-2xl grid-cols-1 items-center gap-x-8 gap-y-16 px-4 lg:max-w-8xl lg:grid-cols-2 lg:px-8 xl:gap-x-16 xl:px-12">
                    <div class="relative z-10 md:text-center lg:text-left">
                        // <Image
                        // class="absolute bottom-full right-full -mb-56 -mr-72 opacity-50"
                        // src={blurCyanImage}
                        // alt=""
                        // width={530}
                        // height={530}
                        // unoptimized
                        // priority
                        // />
                        <div class="relative">
                            <p class="inline bg-gradient-to-r from-indigo-200 via-sky-400 to-indigo-200 bg-clip-text font-display text-5xl tracking-tight text-transparent">
                                "Never miss the cache again."
                            </p>
                            <p class="mt-3 text-2xl tracking-tight text-slate-400">
                                "Cache every single thing your app could ever do ahead of time,
                                so your code never even has to run at all."
                            </p>
                            <div class="mt-8 flex gap-4 md:justify-center lg:justify-start">// <Button href="/">Get started</Button>
                            // <Button href="/" variant="secondary">
                            // View on GitHub
                            // </Button>
                            </div>
                        </div>
                    </div>
                    <div class="relative lg:static xl:pl-10">
                        <div class="absolute inset-x-[-50vw] -bottom-48 -top-32 [mask-image:linear-gradient(transparent,white,white)] dark:[mask-image:linear-gradient(transparent,white,transparent)] lg:-bottom-32 lg:-top-32 lg:left-[calc(50%+14rem)] lg:right-0 lg:[mask-image:none] lg:dark:[mask-image:linear-gradient(white,white,transparent)]">// <HeroBackground class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 lg:left-0 lg:translate-x-0 lg:translate-y-[-60%]" />
                        </div>
                        <div class="relative">
                            // <Image
                            // class="absolute -right-64 -top-64"
                            // src={blurCyanImage}
                            // alt=""
                            // width={530}
                            // height={530}
                            // unoptimized
                            // priority
                            // />
                            // <Image
                            // class="absolute -bottom-40 -right-44"
                            // src={blurIndigoImage}
                            // alt=""
                            // width={567}
                            // height={567}
                            // unoptimized
                            // priority
                            // />
                            <div class="absolute inset-0 rounded-2xl bg-gradient-to-tr from-sky-300 via-sky-300/70 to-blue-300 opacity-10 blur-lg"></div>
                            <div class="absolute inset-0 rounded-2xl bg-gradient-to-tr from-sky-300 via-sky-300/70 to-blue-300 opacity-10"></div>
                            <div class="relative rounded-2xl bg-[#0A101F]/80 ring-1 ring-white/10 backdrop-blur">
                                <div class="absolute -top-px left-20 right-11 h-px bg-gradient-to-r from-sky-300/0 via-sky-300/70 to-sky-300/0"></div>
                                <div class="absolute -bottom-px left-11 right-20 h-px bg-gradient-to-r from-blue-400/0 via-blue-400 to-blue-400/0"></div>
                                <div class="pl-4 pt-4">
                                    // <TrafficLightsIcon class="h-2.5 w-auto stroke-slate-500/30" />
                                    <div class="mt-4 flex space-x-2 text-xs">// {tabs.map((tab) => (
                                    // <div
                                    // key={tab.name}
                                    // class={clsx(
                                    // 'flex h-6 rounded-full',
                                    // tab.isActive
                                    // ? 'bg-gradient-to-r from-sky-400/30 via-sky-400 to-sky-400/30 p-px font-medium text-sky-300'
                                    // : 'text-slate-500',
                                    // )}
                                    // >
                                    // <div
                                    // class={clsx(
                                    // 'flex items-center rounded-full px-2.5',
                                    // tab.isActive && 'bg-slate-800',
                                    // )}
                                    // >
                                    // {tab.name}
                                    // </div>
                                    // </div>
                                    // ))}
                                    </div>
                                // <div class="mt-6 flex items-start px-1 text-sm">
                                // <div
                                // aria-hidden="true"
                                // class="select-none border-r border-slate-300/5 pr-4 font-mono text-slate-600"
                                // >
                                // {Array.from({
                                // length: code.split('\n').length,
                                // }).map((_, index) => (
                                // <Fragment key={index}>
                                // {(index + 1).toString().padStart(2, '0')}
                                // <br />
                                // </Fragment>
                                // ))}
                                // </div>
                                // <Highlight
                                // code={code}
                                // language={codeLanguage}
                                // theme={{ plain: {}, styles: [] }}
                                // >
                                // {({
                                // class,
                                // style,
                                // tokens,
                                // getLineProps,
                                // getTokenProps,
                                // }) => (
                                // <pre
                                // class={clsx(
                                // class,
                                // 'flex overflow-x-auto pb-6',
                                // )}
                                // style={style}
                                // >
                                // <code class="px-4">
                                // {tokens.map((line, lineIndex) => (
                                // <div key={lineIndex} {...getLineProps({ line })}>
                                // {line.map((token, tokenIndex) => (
                                // <span
                                // key={tokenIndex}
                                // {...getTokenProps({ token })}
                                // />
                                // ))}
                                // </div>
                                // ))}
                                // </code>
                                // </pre>
                                // )}
                                // </Highlight>
                                // </div>

                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
