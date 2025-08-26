use dioxus::prelude::*;

use crate::{components::message::Message, datastructures::ChatChannel};

#[component]
pub fn PeonyMessagesView(channel : Option<ChatChannel>) -> Element {
    rsx! {
        if let Some(channel) = channel {
            div {
                class: "sidebar-messages",
                
                div {
                    class: "sidebar-messages-header",
                    "# {channel.name}"
                    div {
                        class: "sidebar-messages-subheader",
                        "Messages for #{channel.name}."
                    }
                }
                ol {
                    class: "list peony-scrollbar peony-scrollbar",
                    for _x in 1..100 {
                        li {
                            Message {
                                display_name: "Tickle Monster",
                                timestamp: "1:06 PM",
                                message_content: "I'm here to tickle you."
                            }
                        }
                    }
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