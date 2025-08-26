use dioxus::prelude::*;
use crate::datastructures::*;

#[component]
pub fn PeonyServerList() -> Element {
    let servers = use_context::<PeonyData>().servers;

    rsx! {
        div {
            class: "sidebar-servers",
            Link {
                class: "linkable",
                to: "/",

                div {
                    class: "server-icon",
                    "Fr",
                }
            }

            hr {}

            for server in servers {
                Link {
                    class: "linkable",
                    to: "/servers/{server.did}/channels/{server.channels[0].id}",
                                

                    div {
                        key: "{server.did}",
                        class: "server-icon",
                        "{server.name.chars().next().unwrap().to_uppercase()}"
                    }
                }
            }
        }
    }
}