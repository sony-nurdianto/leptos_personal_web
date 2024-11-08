use leptos::{component, view, IntoView, RwSignal, Show, SignalGet};
use thaw::{Button, ButtonVariant, Flex, FlexGap, FlexJustify, Icon, Image};

use stylance::import_crate_style;

#[component]
pub fn HomePage(isdark: RwSignal<bool>) -> impl IntoView {
    import_crate_style!(style, "src/styles/main.module.scss");
    view! {
        <Flex vertical=true gap=FlexGap::WH(50, 0)>
            <Flex vertical=true justify=FlexJustify::Center>
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
                </div>
                <div>
                    <Button variant=ButtonVariant::Primary>"Download Cv"</Button>
                </div>
            </Flex>
            <Flex vertical=true>
                <Flex justify=FlexJustify::Center>
                    <h1>"My Digital Toolbox:"</h1>
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Icon style="font-size: 3vw" icon=icondata::SiRust />
                    <Icon style="font-size: 3vw" icon=icondata::BiTypescript />
                    <Icon style="font-size: 3vw" icon=icondata::BiHtml5 />
                    <Icon style="font-size: 3vw" icon=icondata::BiCss3 />
                    <Icon style="font-size: 3vw" icon=icondata::SiDocker />
                    <Icon style="font-size: 3vw" icon=icondata::SiKubernetes />
                </Flex>
            </Flex>
        </Flex>
    }
}
