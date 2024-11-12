use leptos::{component, view, IntoView, RwSignal, Show, SignalGet};
use thaw::{Avatar, Button, Divider, Flex, FlexAlign, FlexGap, FlexJustify, Icon, Image};

use stylance::import_crate_style;

#[component]
pub fn HomePage(isdark: RwSignal<bool>) -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    view! {
        <Flex
            style="
            padding: 3vw"
            vertical=true
            // gap=FlexGap::WH(40, 0)
            align=FlexAlign::Center
        >
            <Flex
                style="line-height: 2vw"
                vertical=true
                justify=FlexJustify::Center
                align=FlexAlign::Center
            >
                <Flex align=FlexAlign::Center>
                    <h1 style="margin: 0 1vw 0 0; font-size: 2vw;">
                        "Rustacean FullStack Engineer "
                    </h1>
                    <Show
                        when=move || isdark.get()
                        fallback=move || {
                            view! {
                                <Image
                                    src="../../../../assets/ferris.svg"
                                    alt="ferris the crab"
                                    height="4vw"
                                    width="4vw"
                                />
                            }
                        }
                    >
                        <Image
                            src="../../../../assets/corro.svg"
                            alt="Coro the Uni"
                            height="4vw"
                            width="4vw"
                        />
                    </Show>
                </Flex>
                <h1 style="color: #FF8000;font-size: 2vw; margin: 0 0 1vw 0;">
                    "On a Journey to Build Better Technology for a Future Beyond Boundaries."
                </h1>
            </Flex>
            <Flex>
                <Button style="
                width: 10vw; 
                height:2.5vw;
                font-size: 1vw;
                box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
                ">
                    "Download Cv"
                </Button>
            </Flex>
            <hr style="width: 100vw; margin-left: -50px; margin-right: -50px; margin-top: 2vw"/>
            <Flex vertical=true>
                <Flex justify=FlexJustify::Center style="font-size: 0.8vw;">
                    <h1 style="margin: 0">"Tech ToolBox :"</h1>
                </Flex>
                <Flex
                    style="
                    width: 100vw;
                    margin-left: -50px;
                    margin-right: -50px;
                    padding-top: 1vw;
                    padding-bottom: 1vw;
                    background-color: #219B9D"
                    justify=FlexJustify::Center
                >
                    <Flex
                        style="width:50vw;"
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Center
                    >
                        <Icon style="font-size: 3vw; color:#EEEEEE" icon=icondata::SiRust />
                        <Icon style="font-size: 3.5vw; color:#EEEEEE" icon=icondata::BiTypescript />
                        <Icon style="font-size: 3.5vw; color:#EEEEEE" icon=icondata::BiHtml5 />
                        <Icon style="font-size: 3.5vw; color:#EEEEEE" icon=icondata::BiCss3 />
                        <Icon style="font-size: 3.5vw; color:#EEEEEE" icon=icondata::SiDocker />
                        <Icon style="font-size: 3vw; color:#EEEEEE" icon=icondata::SiKubernetes />
                    </Flex>
                </Flex>
            </Flex>
            <hr style="width: 100vw; margin-left: -50px; margin-right: -50px;  margin-top: 2.5vw" />
        </Flex>
    }
}
