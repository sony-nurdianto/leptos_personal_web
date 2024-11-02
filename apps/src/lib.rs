mod pages;

use crate::pages::{HomePage, NotFoundPage};
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/personal_web.css" />
        <Title text="Sony Nurdianto" />
        <Router>
            <Routes>
                <Route path="" view=HomePage />
                <Route path="/*any" view=NotFoundPage />
            </Routes>
        </Router>
    }
}

