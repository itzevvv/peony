use dioxus::prelude::*;
use crate::components::friends_list::PeonyFriendsList;

#[component]
pub fn PeonyDMsView(user_did : String) -> Element {
    rsx! {
        PeonyFriendsList { }
        div {
            p { "Messages for {user_did} go here!" }
        }
    }
}