use icondata_core::IconData;
use leptos::{
    create_effect, create_rw_signal, create_signal, ev::MouseEvent, view, For, Show, SignalGet,
    SignalUpdate, SignalWith, View,
};
use leptos_router::use_location;
use thaw::Icon;

use stylance::import_crate_style;

#[derive(Debug, Clone)]
pub struct NavData {
    pub path: &'static str,
    pub icon_active: &'static IconData,
    pub icon_inactive: &'static IconData,
}

impl NavData {
    pub fn get_nav(data: Vec<Self>) -> View {
        import_crate_style!(style, "src/styles/main.module.scss");

        let initial_state = data
            .into_iter()
            .map(|nav_data| (nav_data, create_rw_signal(false)))
            .collect::<Vec<_>>();

        let (nav_state, _) = create_signal(initial_state);

        create_effect(move |_| {
            let loc = use_location();
            nav_state.with(|state_data| {
                for state in state_data {
                    if state.0.path != loc.pathname.get() {
                        state.1.update(|active| *active = false);
                    } else {
                        state.1.update(|active| *active = true);
                    }
                }
            })
        });

        view! {
            <For
                each=move || nav_state.get()
                key=|state| state.0.path
                children=move |(iter_data_nav, active)| {
                    let click_event = move |_: MouseEvent| {
                        let navigate = leptos_router::use_navigate();
                        navigate(iter_data_nav.path, Default::default());
                    };
                    view! {
                        <Show
                            when=move || active.get()
                            fallback=move || {
                                view! {
                                    <button class=style::button_nav on:click=click_event>
                                        <Icon
                                            style="color: black;font-size: 2vw"
                                            icon=iter_data_nav.icon_inactive
                                        />
                                    </button>
                                }
                            }
                        >
                            <button class=style::button_nav on:click=click_event>
                                <Icon
                                    style="color: #06D001; font-size: 2vw"
                                    icon=iter_data_nav.icon_active
                                />
                            </button>
                        </Show>
                    }
                }
            />
        }
    }
}
