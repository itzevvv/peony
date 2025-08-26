use dioxus::prelude::*;
use crate::datastructures::*;

#[component]
pub fn PeonyFriendsList() -> Element {
    let friends = use_context::<PeonyData>().friends;

    rsx! {
        div {
            class: "sidebar-channels",
            h2 {
                "DMs"
            }

            for friend in &friends {
                Link {
                    class: "linkable",
                    to: "/messages/{friend.did}",

                    div {
                        key: "{friend.did}",
                        class: "channel-name",
                        p { "{friend.display_name}" }
                    }
                }
            }
        }
    }
}