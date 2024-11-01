use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Rustacean FullStack Engineer"</h1>
    }
}

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <h1>"Not Found"</h1>
    }
}