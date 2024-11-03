mod views;

use leptos::{
    component, create_effect, create_rw_signal, create_signal, ev::MouseEvent, view, IntoView,
    Show, SignalGet, SignalUpdate, SignalWith,
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use thaw::{Flex, GlobalStyle, Icon, Theme, ThemeProvider};
use views::{
    components::navigation::NavComponent,
    pages::{
        homepage::HomePage, not_found::NotFoundPage, portofolio::PortofolioPage,
        profile::ProfilePage,
    },
};

use stylance::import_crate_style;

#[component]
pub fn App() -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    let theme = create_rw_signal(Theme::light());
    let (dark, set_dark) = create_signal(false);

    create_effect(move |_| {
        theme.update(|t| {
            t.common.background_color = "#F8F4EC".to_string();
            t.common.font_color = "black".to_string();
        });
    });

    let dark_signal = move |_: MouseEvent| {
        set_dark.update(|dark| *dark = !*dark);
        dark.with(|dark| {
            if *dark {
                theme.update(|t| {
                    t.common.background_color = "#021526".to_string();
                    t.common.font_color = "#fff".to_string();
                });
                // theme.set(Theme::dark())
            } else {
                theme.update(|t| {
                    t.common.background_color = "#F8F4EC".to_string();
                    t.common.font_color = "black".to_string();
                });
                // theme.set(Theme::light())
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
                        <NavComponent />
                        <Show
                            when=move || dark.get()
                            fallback=move || {
                                view! {
                                    <button
                                        on:click=dark_signal
                                        class=style::button_nav
                                        style="color: orangered"
                                    >
                                        <Icon icon=icondata::OcSunSm />
                                    </button>
                                }
                            }
                        >
                            <button
                                on:click=dark_signal
                                class=style::button_nav
                                style="color: #03c6fc"
                            >
                                <Icon icon=icondata::BsMoonStars />
                            </button>

                        </Show>
                    </Flex>
                </nav>
            </ThemeProvider>
        </Router>
    }
}
