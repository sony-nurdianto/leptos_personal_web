use leptos::{component, view, IntoView};
use thaw::{Button, ButtonVariant};

use stylance::import_crate_style;

#[component]
pub fn HomePage() -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    view! {
        <div>
            <h1>
                <strong>"Rustacean FullStack Engineer"</strong>
            </h1>
            <h3 class={style::test}>"On a Journey to Build Better Technology for a Future Beyond Boundaries."</h3>
            <Button
                variant=ButtonVariant::Primary
            >
            "Download Cv"
            </Button>
        </div>
    }
}

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <h1>"Not Found"</h1> }
}
