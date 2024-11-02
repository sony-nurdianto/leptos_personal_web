use leptos::{component, view, IntoView, ReadSignal, Show, SignalGet};
use thaw::{Button, ButtonVariant, Icon, Image};

use stylance::import_crate_style;

#[component]
pub fn HomePage(isdark: ReadSignal<bool>) -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    view! {
        <div>
            <h1 style="display: flex; align-items: center;">
                <span style="margin-right: 20px;">"Rustacean FullStack Engineer"</span>
                <Show
                    when=move || isdark.get()
                    fallback=move || {
                        view! {
                            <Image
                                src="../../../../assets/ferris.svg"
                                alt="ferris the crab"
                                height="12%"
                                width="12%"
                            />
                        }
                    }
                >
                    <Image
                        src="../../../../assets/corro.svg"
                        alt="Coro the Uni"
                        height="75px"
                        width="75px"
                    />
                </Show>

            </h1>
            <h2 class=style::test>
                "On a Journey to Build Better Technology for a Future Beyond Boundaries."
            </h2>
            <Button variant=ButtonVariant::Primary>"Download Cv"</Button>
        </div>
    }
}
