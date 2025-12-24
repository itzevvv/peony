use dioxus::prelude::*;

use crate::datastructures::ChatChannel;

#[component]
pub fn PeonyServerChannelHeader(channel : Option<ChatChannel>) -> Element {
    rsx! {
        if let Some(channel) = channel {
            div {
                class: "sidebar-messages-header",
                h1 {
                    class: "sidebar-messages-channel-title",
                    "#{channel.name}"
                }
                div {
                    class: "sidebar-messages-channel-message",
                    "Messages for #{channel.name}."
                }
            }
        } else {
            div {
                h2 { "# Something fucked up!" }
                p { "Something fucked up." }
            },
            p { "You shouldn't be seeing this!" }
        }
    }
}