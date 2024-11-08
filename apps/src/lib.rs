mod views;

use leptos::{component, create_effect, create_rw_signal, view, IntoView, SignalGet};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use thaw::{GlobalStyle, Theme, ThemeProvider};
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
    let dark = create_rw_signal(false);

    create_effect(move |_| {
        dark.get();
    });

    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/personal_web.css" />
        <Title text="Sony Nurdianto" />
        <Router>
            <main>
                <ThemeProvider theme>
                    <GlobalStyle />
                    <Routes>
                        <Route path="" view=move || view! { <HomePage isdark=dark /> } />
                        <Route path="/profile" view=ProfilePage />
                        <Route path="/portofolio" view=PortofolioPage />
                        <Route path="/*any" view=NotFoundPage />
                    </Routes>
                </ThemeProvider>
            </main>
            <NavComponent theme=theme theme_signal=dark />
        </Router>
    }
}
