use leptos::{component, IntoView};
use nav_data::NavData;

pub mod nav_data;

use stylance::import_crate_style;

#[component]
pub fn NavComponent() -> impl IntoView {
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

    NavData::get_nav(nav_items)
}
