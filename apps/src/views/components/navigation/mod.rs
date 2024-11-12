use leptos::{
    component, create_effect, ev::MouseEvent, view, IntoView, RwSignal, Show, SignalGet,
    SignalUpdate, SignalWith,
};
use nav_data::NavData;

pub mod nav_data;

use stylance::import_crate_style;
use thaw::{Flex, FlexJustify, Icon, Theme};

#[component]
pub fn NavComponent(theme: RwSignal<Theme>, theme_signal: RwSignal<bool>) -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");

    let nav_items: Vec<_> = vec![
        NavData {
            path: "/",
            icon_active: icondata::RiHomeSmileBuildingsLine,
            icon_inactive: icondata::AiHomeOutlined,
        },
        NavData {
            path: "/profile",
            icon_active: icondata::CgSmileMouthOpen,
            icon_inactive: icondata::SiCodeproject,
        },
        NavData {
            path: "/portofolio",
            icon_active: icondata::AiExperimentFilled,
            icon_inactive: icondata::AiExperimentTwotone,
        },
    ];

    create_effect(move |_| {
        theme.update(|t| {
            t.common.background_color = "#EEEEEE".to_string();
            t.common.font_color = "black".to_string();
            t.common.color_primary = "#563A9C".to_string();
            t.common.color_primary_hover = "#4C1F7A".to_string();
            t.common.color_primary_active = "#563A9C".to_string();
        });
    });

    let dark_signal = move |_: MouseEvent| {
        theme_signal.update(|dark| *dark = !*dark);
        theme_signal.with(|dark| {
            if *dark {
                theme.update(|t| {
                    t.common.background_color = "#021526".to_string();
                    t.common.font_color = "#fff".to_string();
                });
            } else {
                theme.update(|t| {
                    t.common.background_color = "#F8F4EC".to_string();
                    t.common.font_color = "black".to_string();
                });
            }
        });
    };

    view! {
        <nav class=style::floating_navi>
            <Flex justify=FlexJustify::SpaceAround>
                {NavData::get_nav(nav_items)}
                <Show
                    when=move || theme_signal.get()
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
                    <button on:click=dark_signal class=style::button_nav style="color: #03c6fc">
                        <Icon icon=icondata::BsMoonStars />
                    </button>

                </Show>
            </Flex>
        </nav>
    }
}
