use dioxus::prelude::*;
use crate::{components::friends_list::PeonyFriendsList, datastructures::PeonyData};

#[component]
pub fn PeonyFriendsView() -> Element {
    let friends = use_context::<PeonyData>().friends;

    rsx! {
        PeonyFriendsList { }
        div {
            h3 { "Friends" }
            for friend in &friends {
                div {
                    key: "{friend.did}",
                    class: "channel-name",
                    p { "{friend.display_name}" }
                }
            }
        }
    }
}