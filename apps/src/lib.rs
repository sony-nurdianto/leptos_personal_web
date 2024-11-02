mod views;

use leptos::{
    component, create_rw_signal, create_signal, view, IntoView, Show, SignalGet, SignalSet,
    SignalUpdate, SignalWith,
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use thaw::{Flex, GlobalStyle, Icon, Theme, ThemeProvider};
use views::pages::{
    homepage::HomePage, not_found::NotFoundPage, portofolio::PortofolioPage, profile::ProfilePage,
};

use stylance::import_crate_style;

#[component]
pub fn App() -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    let theme = create_rw_signal(Theme::light());
    let (dark, set_dark) = create_signal(false);

    let dark_signal = move |_| {
        set_dark.update(|dark| *dark = !*dark);
        dark.with(|dark| {
            if *dark {
                theme.set(Theme::dark())
            } else {
                theme.set(Theme::light())
            }
        });
    };

    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/personal_web.css" />
        <Title text="Sony Nurdianto" />
        <Router>
            <ThemeProvider theme>
                <GlobalStyle />
                <main>
                    <Routes>
                        <Route path="" view=move || view! { <HomePage isdark=dark /> } />
                        <Route path="/profile" view=ProfilePage />
                        <Route path="/portofolio" view=PortofolioPage />
                        <Route path="/*any" view=NotFoundPage />
                    </Routes>
                </main>
                <nav class=style::floating_navi>
                    <Flex>
                        <button
                            on:click=move |_| {
                                let navigate = leptos_router::use_navigate();
                                navigate("", Default::default());
                            }

                            class=style::button_nav
                        >
                            <Icon icon=icondata::IoHomeOutline />
                        </button>
                        <button
                            on:click=move |_| {
                                let navigate = leptos_router::use_navigate();
                                navigate("/profile", Default::default());
                            }

                            class=style::button_nav
                        >
                            <Icon icon=icondata::SiCodeproject />
                        </button>
                        <button
                            on:click=move |_| {
                                let navigate = leptos_router::use_navigate();
                                navigate("/portofolio", Default::default());
                            }

                            class=style::button_nav
                        >
                            <Icon icon=icondata::AiExperimentTwotone />
                        </button>

                        <Show
                            when=move || dark.get()
                            fallback=move || {
                                view! {
                                    <button on:click=dark_signal class=style::button_nav>
                                        <Icon icon=icondata::BsMoonStars />
                                    </button>
                                }
                            }
                        >
                            <button on:click=dark_signal class=style::button_nav>
                                <Icon icon=icondata::OcSunSm />

                            </button>

                        </Show>
                    </Flex>
                </nav>
            </ThemeProvider>
        </Router>
    }
}
